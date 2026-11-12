use anyhow::{Context, Result, bail};
use serde_yaml_ng::Value;
use std::fs;
use std::path::Path;

use super::build::build_project;
use super::obscure::obscure_project;
use crate::cli::{DeployArgs, ObscureLevel};
use crate::mantle::{self, Settings};
use crate::{exec, log, project};

/// Where the build sits relative to `artifacts/<name>/<environment>/`
const PLACE_FILE: &str = "../build.rbxl";

pub fn run(args: &DeployArgs) -> Result<()> {
    let (root, names) = project::select(
        args.project.name.as_deref(),
        args.project.target.as_deref(),
        args.project.all,
        "deploy",
    )?;

    let settings = Settings {
        environment: &args.environment,
        group_id: args.group_id,
        payments: args.payments.as_deref(),
        place_name: args.place_name.as_deref(),
    };

    project::run_each(&names, "deploy", |name| {
        deploy_project(&root, name, &settings, args.level)
    })
}

pub fn deploy_project(
    root: &Path,
    name: &str,
    settings: &Settings,
    level: Option<ObscureLevel>,
) -> Result<()> {
    project::project_dir(root, name)?;

    match level {
        Some(level) => {
            log::info(format!("Building project \"{name}\" (obscured) before deploying..."));
            obscure_project(root, name, level)?;
        }
        None => {
            log::info(format!("Building project \"{name}\" before deploying..."));
            build_project(root, name)?;
        }
    }

    let artifacts = project::artifacts_dir(root, name);
    let build_file = artifacts.join("build.rbxl");
    if !build_file.is_file() {
        bail!("Build output \"{}\" is missing after building \"{name}\".", build_file.display());
    }

    let mantle_dir = artifacts.join(settings.environment);
    fs::create_dir_all(&mantle_dir)?;

    let mantle_file = mantle_dir.join("mantle.yml");
    let mut config = if mantle_file.is_file() {
        log::verbose(format!("Updating existing mantle.yml at \"{}\"...", mantle_file.display()));
        let text = fs::read_to_string(&mantle_file)?;
        serde_yaml_ng::from_str(&text)
            .with_context(|| format!("\"{}\" isn't valid YAML", mantle_file.display()))?
    } else {
        log::verbose(format!("Creating new mantle.yml at \"{}\"...", mantle_file.display()));
        Value::Null
    };

    mantle::apply(&mut config, PLACE_FILE, settings);

    if !mantle::has_owner(&config) {
        log::warn(format!(
            "No group ID configured for \"{name}\"; resources will be created under the personal account. Pass --groupid/-g to deploy under a group instead."
        ));
    }

    fs::write(&mantle_file, serde_yaml_ng::to_string(&config)?)?;

    log::info(format!("Deploying project \"{name}\" to environment \"{}\"...", settings.environment));
    exec::run_inherit(
        exec::command("mantle", root)
            .arg("deploy")
            .arg(&mantle_dir)
            .arg("--environment")
            .arg(settings.environment),
    )
}
