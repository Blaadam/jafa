use anyhow::{Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::log;

pub fn workspace_root(target: &str) -> Result<PathBuf> {
    Ok(std::env::current_dir()?.join(target))
}

pub fn projects_dir(root: &Path) -> PathBuf {
    root.join("projects")
}

pub fn artifacts_dir(root: &Path, name: &str) -> PathBuf {
    root.join("artifacts").join(name)
}

pub fn project_dir(root: &Path, name: &str) -> Result<PathBuf> {
    let dir = projects_dir(root).join(name);

    if !dir.is_dir() {
        bail!(
            "\"{name}\" doesn't exist in \"{}\". Make sure the project exists.",
            projects_dir(root).display()
        );
    }

    Ok(dir)
}

pub fn list_projects(root: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(projects_dir(root)) else {
        return Vec::new();
    };

    let mut names: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    names.sort();
    names
}

pub fn select(
    name: Option<&str>,
    target: Option<&str>,
    all: bool,
    verb: &str,
) -> Result<(PathBuf, Vec<String>)> {
    if all {
        // there is no project name with --all so a lone positional is the target
        let root = workspace_root(name.or(target).unwrap_or("."))?;
        let names = list_projects(&root);

        if names.is_empty() {
            bail!("No projects found to {verb}.");
        }

        return Ok((root, names));
    }

    let Some(name) = name else {
        bail!(
            "\"{verb}\" requires a project name, e.g. \"jafa {verb} my-project\", or use -a/--all to {verb} all projects"
        );
    };

    Ok((workspace_root(target.unwrap_or("."))?, vec![name.to_string()]))
}

pub fn run_each(
    names: &[String],
    verb: &str,
    mut f: impl FnMut(&str) -> Result<()>,
) -> Result<()> {
    if let [name] = names {
        return f(name);
    }

    let mut failures = Vec::new();
    for name in names {
        if let Err(err) = f(name) {
            log::error(format!("Failed to {verb} \"{name}\": {err:#}"));
            failures.push(name.as_str());
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        bail!("Failed to {verb}: {}", failures.join(", "))
    }
}
