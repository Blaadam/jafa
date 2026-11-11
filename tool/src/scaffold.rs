use anyhow::{Context, Result};
use include_dir::{Dir, include_dir};
use std::fs;
use std::path::Path;

static SCAFFOLDING: Dir = include_dir!("$CARGO_MANIFEST_DIR/scaffolding");

pub fn copy_project(dest: &Path) -> Result<()> {
    copy_dir("project", dest)
}

pub fn copy_workspace(dest: &Path) -> Result<()> {
    copy_dir("workspace", dest)
}

/// Writes over files from the project scaffolding with ones from `overlays/<name>`
pub fn apply_overlay(name: &str, dest: &Path) -> Result<()> {
    copy_dir(&format!("overlays/{name}"), dest)
}

fn copy_dir(name: &str, dest: &Path) -> Result<()> {
    let dir = SCAFFOLDING
        .get_dir(name)
        .with_context(|| format!("scaffolding \"{name}\" is missing from the binary"))?;

    write_dir(dir, Path::new(name), dest)
}

// embedded paths are relative to the scaffolding root, so `base` is stripped to land them in `dest`
fn write_dir(dir: &Dir, base: &Path, dest: &Path) -> Result<()> {
    for file in dir.files() {
        let out = dest.join(file.path().strip_prefix(base)?);

        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&out, file.contents())
            .with_context(|| format!("failed to write \"{}\"", out.display()))?;
    }

    for sub in dir.dirs() {
        write_dir(sub, base, dest)?;
    }

    Ok(())
}

/// Creates the folders described by a `struct.json`: an object of folder names, nested to any depth
pub fn create_folders(base: &Path, structure: &serde_json::Value) -> Result<()> {
    let Some(folders) = structure.as_object() else {
        return Ok(());
    };

    for (name, nested) in folders {
        let path = base.join(name);
        crate::log::verbose(format!("Creating folder: {}", path.display()));
        fs::create_dir_all(&path)?;
        create_folders(&path, nested)?;
    }

    Ok(())
}
