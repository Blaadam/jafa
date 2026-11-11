use anyhow::Result;
use std::path::Path;

use crate::cli::SingleArgs;
use crate::{exec, log, project};

pub fn run(args: &SingleArgs) -> Result<()> {
    let root = project::workspace_root(&args.target)?;
    serve_project(&root, &args.name)
}

pub fn serve_project(root: &Path, name: &str) -> Result<()> {
    let dir = project::project_dir(root, name)?;

    log::info(format!("Serving project \"{name}\""));
    exec::run_inherit(exec::command("rojo", &dir).arg("serve"))
}
