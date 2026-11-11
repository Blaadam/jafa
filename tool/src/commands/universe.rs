use anyhow::{Context, Result, bail};
use serde_yaml_ng::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use super::build::build_project;
use super::new::COMMON;
use crate::code_workspace::display_name;
use crate::{exec, log, mantle, project};

const BUILD_FILE: &str = "build.rbxl";

pub fn new(name: &str, projects: &[String], environment: &str, target: &str) -> Result<()> {
    project::validate_name(name)?;
    let root = project::workspace_root(target)?;

    let mut projects = projects.to_vec();
    if projects.is_empty() {
        projects = project::list_projects(&root);
        projects.retain(|project| project != COMMON);
    }
    if projects.is_empty() {
        bail!("No projects found to put in the universe.");
    }
    for project in &projects {
        project::project_dir(&root, project)?;
    }

    let file = root.join("universes").join(name).join(environment).join("mantle.yml");
    if file.exists() {
        bail!("\"{}\" already exists.", file.display());
    }

    fs::create_dir_all(file.parent().expect("joined above"))?;
    fs::write(&file, starter_config(name, &projects, environment))?;

    log::info(format!("Created \"{}\". Edit it, then run `jafa deploy-universe {name}`.", file.display()));
    Ok(())
}

pub fn deploy(name: &str, environment: &str, target: &str) -> Result<()> {
    project::validate_name(name)?;
    let root = project::workspace_root(target)?;
    let file = find_config(&root, name, environment)?;

    let text = fs::read_to_string(&file)?;
    let config: Value = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("\"{}\" isn't valid YAML", file.display()))?;
    let places = config
        .get("target")
        .and_then(|target| target.get("experience"))
        .and_then(|experience| experience.get("places"))
        .and_then(Value::as_mapping)
        .with_context(|| format!("\"{}\" has no target.experience.places", file.display()))?;

    let root = normalize(&root);
    let dir = file.parent().expect("a file has a parent");
    let mut built = HashSet::new();

    for (key, place) in places {
        let key = key.as_str().unwrap_or("?");
        let Some(place_file) = place.get("file").and_then(Value::as_str) else {
            bail!("place \"{key}\" in \"{}\" has no file", file.display());
        };

        let path = normalize(&dir.join(place_file));
        if let Some(project) = built_project(&root, &path)
            && project::project_dir(&root, &project).is_ok()
            && built.insert(project.clone())
        {
            log::info(format!("Building \"{project}\" for place \"{key}\"..."));
            build_project(&root, &project)?;
        }

        if !path.is_file() {
            bail!("place \"{key}\" needs \"{}\", but it doesn't exist.", path.display());
        }
    }

    log::info(format!("Deploying universe \"{name}\" to environment \"{environment}\"..."));
    exec::run_inherit(
        exec::command("mantle", &root)
            .arg("deploy")
            .arg(&file)
            .arg("--environment")
            .arg(environment),
    )
}

fn find_config(root: &Path, name: &str, environment: &str) -> Result<PathBuf> {
    let mut candidates = vec![root.join("universes")];
    // a project's own generated mantle.yml lives here too, so it isn't a universe
    if !project::projects_dir(root).join(name).is_dir() {
        candidates.push(root.join("artifacts"));
    }

    candidates
        .into_iter()
        .map(|base| base.join(name).join(environment).join("mantle.yml"))
        .find(|file| file.is_file())
        .with_context(|| {
            format!(
                "No universe \"{name}\" for \"{environment}\". Create one with `jafa new-universe {name} -e {environment}`."
            )
        })
}

/// `<root>/artifacts/<project>/build.rbxl` -> `<project>`
fn built_project(root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(root.join("artifacts")).ok()?;
    let mut parts = relative.components();
    let project = parts.next()?.as_os_str().to_str()?.to_string();

    (parts.next()?.as_os_str() == BUILD_FILE && parts.next().is_none()).then_some(project)
}

/// Resolves `.` and `..` by name, since the build may not exist yet to canonicalize
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

fn starter_config(name: &str, projects: &[String], environment: &str) -> String {
    let mut text = format!("# Universe \"{name}\": one experience with a place per project.\n");
    text.push_str("# Add badges, icons and the rest from https://mantle-docs.vercel.app\n");
    text.push_str("# owner:\n#   group: 0\n\n");
    text.push_str("target:\n  experience:\n    places:\n");

    for (index, project) in projects.iter().enumerate() {
        let key = if index == 0 { "start" } else { project };
        text.push_str(&format!(
            "      {key}:\n        file: ../../../artifacts/{project}/{BUILD_FILE}\n        configuration:\n          name: {}\n",
            display_name(project)
        ));
    }

    text.push_str(&format!("\nenvironments:\n  - label: {environment}\n    targetAccess: private\n"));
    if mantle::uses_label_prefix(environment) {
        text.push_str("    targetNamePrefix: environmentLabel\n");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_resolves_dots() {
        assert_eq!(normalize(Path::new("a/b/../../c/./d")), PathBuf::from("c/d"));
    }

    #[test]
    fn finds_the_project_behind_a_build() {
        let root = Path::new("w");
        assert_eq!(built_project(root, Path::new("w/artifacts/tennis/build.rbxl")).as_deref(), Some("tennis"));
        assert_eq!(built_project(root, Path::new("w/artifacts/tennis/dev/build.rbxl")), None);
        assert_eq!(built_project(root, Path::new("w/assets/build.rbxl")), None);
    }

    #[test]
    fn starter_config_is_valid_and_points_at_builds() {
        let text = starter_config("big", &["lobby".into(), "tennis".into()], "dev");
        let config: Value = serde_yaml_ng::from_str(&text).unwrap();
        let places = &config["target"]["experience"]["places"];

        assert_eq!(places["start"]["file"], "../../../artifacts/lobby/build.rbxl");
        assert_eq!(places["tennis"]["configuration"]["name"], "Tennis");
        assert_eq!(config["environments"][0]["targetNamePrefix"], "environmentLabel");
    }
}
