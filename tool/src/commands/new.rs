use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{code_workspace, log, project, scaffold, workspace_file};

pub fn run(name: &str, target: &str) -> Result<()> {
    project::validate_name(name)?;
    let root = project::workspace_root(target)?;
    create_project(&root, name, true)?;
    Ok(())
}

pub const COMMON: &str = "common";

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

    if name == COMMON {
        log::verbose("Setting up as the shared common project...");
        scaffold::apply_overlay("common", &dest)?;
        // common only holds shared code, so it has no entry points of its own
        let _ = fs::remove_file(dest.join("client/ClientMain.client.luau"));
        let _ = fs::remove_file(dest.join("server/ServerMain.server.luau"));
    } else if projects_dir.join(COMMON).is_dir() {
        log::verbose("Linking to the common project...");
        scaffold::apply_overlay("uses-common", &dest)?;
    }

    set_project_name(&dest, name)?;

    log::info(format!("Creating sub-folders for {name}..."));
    let struct_file = dest.join("struct.json");
    let structure = serde_json::from_str(&fs::read_to_string(&struct_file)?)
        .context("scaffolding struct.json is not valid JSON")?;
    scaffold::create_folders(&dest, &structure)?;
    fs::remove_file(&struct_file)?;

    workspace_file::add_project(root, name)?;

    // the project is made by now, so a problem with this file shouldn't fail the command
    if let Err(err) = code_workspace::add_project(root, name) {
        log::warn(format!("Couldn't add \"{name}\" to jafa.code-workspace: {err:#}"));
    }

    log::info(format!(
        "Project \"{name}\" created successfully at \"{}\".",
        dest.display()
    ));

    Ok(dest)
}

// the scaffolding calls every project "project-name", which Rojo shows as the project's name
fn set_project_name(dest: &Path, name: &str) -> Result<()> {
    let file = dest.join("default.project.json");
    let text = fs::read_to_string(&file)?;
    let updated = text.replacen("\"name\": \"project-name\"", &format!("\"name\": \"{name}\""), 1);

    if updated != text {
        fs::write(&file, updated)?;
    }

    Ok(())
}
