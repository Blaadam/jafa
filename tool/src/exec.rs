use anyhow::{Result, bail};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

pub fn command(program: &str, cwd: &Path) -> Command {
    let mut cmd = Command::new(program);
    cmd.current_dir(cwd);
    cmd
}

/// Fails on exit code, not stderr, since rojo and mantle print progress there
pub fn run(cmd: &mut Command) -> Result<String> {
    let program = cmd.get_program().to_string_lossy().into_owned();
    let output = cmd.output().map_err(|err| spawn_error(&program, err))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("`{program}` failed ({}): {}", output.status, stderr.trim());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn run_inherit(cmd: &mut Command) -> Result<()> {
    let program = cmd.get_program().to_string_lossy().into_owned();
    let status = cmd.status().map_err(|err| spawn_error(&program, err))?;

    if !status.success() {
        bail!("`{program}` failed ({status})");
    }

    Ok(())
}

fn spawn_error(program: &str, err: std::io::Error) -> anyhow::Error {
    if err.kind() == ErrorKind::NotFound {
        anyhow::anyhow!("`{program}` was not found. Make sure it is installed and on your PATH (try `rokit install`)")
    } else {
        anyhow::anyhow!("failed to run `{program}`: {err}")
    }
}
