use anyhow::{Result, bail};
use std::fs;
use std::path::Path;

use super::new::{COMMON, create_project};
use crate::{exec, log, project, prompt, scaffold, workspace_file};

pub fn run(target: &str, yes: bool, common: bool) -> Result<()> {
    let root = project::workspace_root(target)?;
    init_workspace(&root, yes, common)
}

pub fn init_workspace(root: &Path, yes: bool, common: bool) -> Result<()> {
    if workspace_file::path(root).exists() {
        bail!("\"{}\" is already a jafa workspace.", root.display());
    }

    let ask = |question: &str| yes || prompt::confirm(question, true);
    let run_git = !root.join(".git").exists() && ask("Run `git init`?");
    let run_rokit = ask("Run `rokit install`?");
    let run_wally = ask("Run `wally install`?");
    // --yes doesn't opt in to this, since it's optional extra structure
    let with_common = common || (!yes && prompt::confirm("Create a shared `common` project?", false));

    log::info(format!("Initializing jafa in {}", root.display()));
    fs::create_dir_all(root)?;
    scaffold::copy_workspace(root)?;

    if run_git {
        log::verbose("Initializing git repository...");
        setup_step(exec::run(exec::command("git", root).arg("init")).map(|_| ()), "git init");
    } else {
        log::verbose("Skipping git init.");
    }

    if run_rokit {
        log::info("Installing dependencies...");
        setup_step(
            exec::run_inherit(exec::command("rokit", root).arg("install")),
            "rokit install",
        );
    } else {
        log::verbose("Skipping rokit install.");
    }

    if run_wally {
        log::verbose("Installing wally packages");
        setup_step(
            exec::run_inherit(
                exec::command("wally", root)
                    .arg("install")
                    .arg("--project-path")
                    .arg("./wally"),
            ),
            "wally install --project-path ./wally",
        );
    } else {
        log::verbose("Skipping wally install.");
    }

    log::info("Creating workspace file...");
    workspace_file::create(root)?;

    if with_common {
        log::verbose("Creating common project...");
        create_project(root, COMMON, false)?;
    }

    log::verbose("Creating project...");
    create_project(root, "project-name", false)?;

    log::info("Workspace created successfully.");
    Ok(())
}

// the workspace is still usable if a tool fails, so warn and carry on
fn setup_step(result: Result<()>, command: &str) {
    if let Err(err) = result {
        log::warn(format!("`{command}` failed, run it yourself later: {err:#}"));
    }
}
