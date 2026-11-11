use anyhow::Result;
use std::path::Path;

use super::build::build_project;
use super::serve::serve_project;
use super::start::start_project;

use crate::cli::SingleArgs;
use crate::{log, project};

pub fn run(args: &SingleArgs) -> Result<()> {
    let root = project::workspace_root(&args.target)?;
    dev_project(&root, &args.name)
}

pub fn dev_project(root: &Path, name: &str) -> Result<()> {
    log::info(format!("Starting Dev for project \"{name}\"..."));

    build_project(root, name)?;
    start_project(root, name)?;
    serve_project(root, name)?;

    Ok(())
}
