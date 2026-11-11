use anyhow::Result;
use std::path::Path;

use crate::cli::ProjectArgs;
use crate::{exec, log, project};

pub fn run(args: &ProjectArgs) -> Result<()> {
    let (root, names) = project::select(
        args.name.as_deref(),
        args.target.as_deref(),
        args.all,
        "sourcemap",
    )?;

    project::run_each(&names, "sourcemap", |name| sourcemap_project(&root, name))
}

pub fn sourcemap_project(root: &Path, name: &str) -> Result<()> {
    let dir = project::project_dir(root, name)?;

    log::info(format!("Building sourcemap for \"{name}\""));
    exec::run(
        exec::command("rojo", &dir)
            .arg("sourcemap")
            .arg("-o")
            .arg(dir.join("sourcemap.json")),
    )?;

    Ok(())
}
