use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::log;

#[derive(Serialize, Deserialize)]
pub struct WorkspaceProject {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFile {
    pub created: String,
    pub projects_path: String,
    #[serde(default)]
    pub projects: Vec<WorkspaceProject>,
}

pub fn path(root: &Path) -> PathBuf {
    root.join(".vscode").join(".jafa-workspace")
}

pub fn create(root: &Path) -> Result<()> {
    log::verbose("Creating jafa-workspace file...");
    fs::create_dir_all(root.join(".vscode"))?;

    let now = OffsetDateTime::now_utc();
    let now = now.replace_nanosecond(now.nanosecond() / 1_000_000 * 1_000_000)?;

    write(
        root,
        &WorkspaceFile {
            created: now.format(&Rfc3339)?,
            projects_path: "jafa-workspace://projects".to_string(),
            projects: Vec::new(),
        },
    )
}

pub fn read(root: &Path) -> Result<WorkspaceFile> {
    let file = path(root);
    if !file.is_file() {
        bail!("Workspace file \"{}\" does not exist.", file.display());
    }

    let contents = fs::read_to_string(&file)?;
    serde_json::from_str(&contents)
        .with_context(|| format!("\"{}\" is not a valid jafa workspace file", file.display()))
}

pub fn write(root: &Path, workspace: &WorkspaceFile) -> Result<()> {
    let file = path(root);
    fs::write(&file, serde_json::to_string(workspace)? + "\n")
        .with_context(|| format!("failed to write \"{}\"", file.display()))
}

pub fn add_project(root: &Path, name: &str) -> Result<()> {
    let mut workspace = read(root)?;
    let project_path = format!("projects://{name}");

    if let Some(existing) = workspace.projects.iter_mut().find(|p| p.name == name) {
        log::verbose(format!("Updating existing project \"{name}\" in workspace file..."));
        existing.path = project_path;
    } else {
        log::verbose(format!("Adding project \"{name}\" to workspace file..."));
        workspace.projects.push(WorkspaceProject {
            name: name.to_string(),
            path: project_path,
        });
    }

    write(root, &workspace)
}
