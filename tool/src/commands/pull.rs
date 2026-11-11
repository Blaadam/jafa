use anyhow::{Result, bail};
use std::path::Path;

use crate::cli::ProjectArgs;
use crate::{exec, log, project};

const SCRIPT: &str = "asset-updater.luau";

pub fn run(args: &ProjectArgs) -> Result<()> {
    let (root, names) = project::select(
        args.name.as_deref(),
        args.target.as_deref(),
        args.all,
        "pull",
    )?;

    project::run_each(&names, "pull", |name| pull_project(&root, name))
}

pub fn pull_project(root: &Path, name: &str) -> Result<()> {
    project::project_dir(root, name)?;

    if !root.join(SCRIPT).is_file() {
        bail!(
            "\"{SCRIPT}\" doesn't exist in \"{}\". Make sure you're in a jafa workspace.",
            root.display()
        );
    }

    log::info(format!("Pulling assets for project \"{name}\"..."));
    let output = exec::run(
        exec::command("lune", root)
            .arg("run")
            .arg(SCRIPT)
            .arg(name),
    )?;

    if !output.is_empty() {
        log::info(format!("Pull output: {output}"));
    }

    Ok(())
}
