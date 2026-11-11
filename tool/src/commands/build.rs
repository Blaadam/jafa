use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::sourcemap::sourcemap_project;
use crate::cli::ProjectArgs;
use crate::{exec, log, project};

pub fn run(args: &ProjectArgs) -> Result<()> {
    let (root, names) = project::select(
        args.name.as_deref(),
        args.target.as_deref(),
        args.all,
        "build",
    )?;

    project::run_each(&names, "build", |name| build_project(&root, name))
}

pub fn build_project(root: &Path, name: &str) -> Result<()> {
    let dir = project::project_dir(root, name)?;

    let build_dir = project::artifacts_dir(root, name);
    log::verbose(format!("Ensuring build directory exists for {name}..."));
    fs::create_dir_all(&build_dir)
        .with_context(|| format!("failed to create \"{}\"", build_dir.display()))?;

    log::info(format!("Building project \"{name}\"..."));
    let output = exec::run(
        exec::command("rojo", &dir)
            .arg("build")
            .arg("-o")
            .arg(build_dir.join("build.rbxl"))
            .arg(dir.join("default.project.json")),
    )?;

    if !output.is_empty() {
        log::info(format!("Build output: {output}"));
    }

    sourcemap_project(root, name)
}
