use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{log, project, scaffold, workspace_file};

pub fn run(name: &str, target: &str) -> Result<()> {
    let root = project::workspace_root(target)?;
    create_project(&root, name, true)?;
    Ok(())
}

/// `check_exists` is off when `init` creates the first project, since the workspace is brand new
pub fn create_project(root: &Path, name: &str, check_exists: bool) -> Result<PathBuf> {
    let projects_dir = project::projects_dir(root);
    let dest = projects_dir.join(name);

    if check_exists {
        if !projects_dir.is_dir() {
            bail!(
                "\"{}\" doesn't exist. Make sure \"{}\" is a jafa workspace.",
                projects_dir.display(),
                root.display()
            );
        }
        if dest.exists() {
            bail!("\"{}\" already exists", dest.display());
        }
    }

    // fail before copying anything, so a missing workspace file can't leave a half made project
    workspace_file::read(root)?;

    log::info(format!("Creating project \"{name}\"..."));
    fs::create_dir_all(&projects_dir)?;
    scaffold::copy_project(&dest)?;

    log::info(format!("Creating sub-folders for {name}..."));
    let struct_file = dest.join("struct.json");
    let structure = serde_json::from_str(&fs::read_to_string(&struct_file)?)
        .context("scaffolding struct.json is not valid JSON")?;
    scaffold::create_folders(&dest, &structure)?;
    fs::remove_file(&struct_file)?;

    workspace_file::add_project(root, name)?;

    log::info(format!(
        "Project \"{name}\" created successfully at \"{}\".",
        dest.display()
    ));

    Ok(dest)
}
