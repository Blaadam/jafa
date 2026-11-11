use anyhow::{Result, bail};
use std::path::Path;

use crate::cli::ProjectArgs;
use crate::{exec, log, project};

pub fn run(args: &ProjectArgs) -> Result<()> {
    let (root, names) = project::select(
        args.name.as_deref(),
        args.target.as_deref(),
        args.all,
        "open",
    )?;

    project::run_each(&names, "open", |name| start_project(&root, name))
}

pub fn start_project(root: &Path, name: &str) -> Result<()> {
    let build_dir = project::artifacts_dir(root, name);

    log::verbose(format!("Checking if build directory exists for {name}..."));
    if !build_dir.is_dir() {
        bail!(
            "\"{name}\" doesn't exist in \"{}\". Make sure the project exists.",
            root.join("artifacts").display()
        );
    }

    let build_file = build_dir.join("build.rbxl");
    if !build_file.is_file() {
        bail!(
            "Build file \"{}\" does not exist. Make sure the build was successful.",
            build_file.display()
        );
    }

    log::info(format!("Opening build file: {}", build_file.display()));

    let mut cmd = if cfg!(target_os = "windows") {
        // not `cmd /C start`, since cmd would read & | ^ % in the path as its own syntax
        let mut cmd = exec::command("rundll32", root);
        cmd.arg("url.dll,FileProtocolHandler");
        cmd
    } else if cfg!(target_os = "macos") {
        exec::command("open", root)
    } else {
        log::warn("Note that Linux support is limited both for Jafa and Roblox.");
        exec::command("xdg-open", root)
    };

    exec::run(cmd.arg(&build_file))?;
    Ok(())
}
