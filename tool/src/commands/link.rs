use anyhow::{Context, Result, bail};
use jsonc_parser::ParseOptions;
use jsonc_parser::cst::{CstInputValue, CstNode, CstObject, CstRootNode};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::{log, project};

const PROJECT_FILE: &str = "default.project.json";

/// Suffixes Rojo strips from a file's name to get the instance name, longest first
const SUFFIXES: &[&str] = &[
    ".server.luau", ".client.luau", ".server.lua", ".client.lua", ".model.json", ".project.json",
    ".luau", ".lua", ".json", ".txt", ".csv", ".rbxmx", ".rbxm", ".toml", ".yml", ".yaml",
];

/// A file or folder inside a project, found by where its parent is mounted in the Rojo tree
struct Source {
    project: String,
    /// Path inside the project, `/` separated, with the casing it has on disk
    rel: String,
    /// Keys leading from the tree's root to the node that maps the parent folder
    mount: Vec<String>,
    /// Where that node's folder is inside the project
    mount_dir: String,
    /// Instance name Rojo gives it
    name: String,
}

impl Source {
    fn link_path(&self) -> String {
        format!("../{}/{}", self.project, self.rel)
    }
}

pub fn link(root: &Path, source: &str, targets: &[String], name: Option<&str>) -> Result<()> {
    let source = resolve_source(root, source, name, true)?;
    let link_path = source.link_path();

    let mut failed = Vec::new();
    for target in targets {
        match link_into(root, &source, &link_path, target) {
            Ok(true) => log::info(format!("Linked \"{}/{}\" into \"{target}\"", source.project, source.rel)),
            Ok(false) => {}
            Err(err) => {
                log::warn(format!("Skipped \"{target}\": {err:#}"));
                failed.push(target.as_str());
            }
        }
    }

    finish("link", &failed)
}

pub fn unlink(root: &Path, source: &str, targets: &[String], name: Option<&str>) -> Result<()> {
    // the file may have been deleted since it was linked, so it doesn't have to exist
    let source = resolve_source(root, source, name, false)?;
    let link_path = source.link_path();

    let mut failed = Vec::new();
    for target in targets {
        match unlink_from(root, &source, &link_path, target) {
            Ok(true) => log::info(format!("Unlinked \"{}/{}\" from \"{target}\"", source.project, source.rel)),
            Ok(false) => {}
            Err(err) => {
                log::warn(format!("Skipped \"{target}\": {err:#}"));
                failed.push(target.as_str());
            }
        }
    }

    finish("unlink", &failed)
}

fn finish(verb: &str, failed: &[&str]) -> Result<()> {
    if failed.is_empty() {
        Ok(())
    } else {
        bail!("Couldn't {verb} into: {}", failed.join(", "))
    }
}

/// Returns whether anything changed, since a target that already has the link isn't an error
fn link_into(root: &Path, source: &Source, link_path: &str, target: &str) -> Result<bool> {
    project::validate_name(target)?;
    if target == source.project {
        bail!("that's the project the file comes from");
    }

    let dir = project::project_dir(root, target)?;
    let (file, text) = read_project_file(&dir)?;
    let cst = parse(&file, &text)?;
    let node = mount_node(&cst, &source.mount)
        .with_context(|| format!("its project file has no \"{}\" in its tree", source.mount.join(".")))?;

    if let Some(existing) = node.get(&source.name) {
        let linked = path_of(&existing.value()).is_some_and(|path| path == link_path);
        if linked {
            log::info(format!("\"{target}\" already has \"{}\" linked", source.name));
            return Ok(false);
        }
        bail!("its tree already has an entry named \"{}\"", source.name);
    }
    if folder_has(&dir.join(&source.mount_dir), &source.name) {
        bail!("\"{}/{}\" already has something named \"{}\"", target, source.mount_dir, source.name);
    }

    node.append(
        &source.name,
        CstInputValue::Object(vec![("$path".to_string(), link_path.to_string().into())]),
    );
    fs::write(&file, cst.to_string()).with_context(|| format!("failed to write \"{}\"", file.display()))?;
    Ok(true)
}

fn unlink_from(root: &Path, source: &Source, link_path: &str, target: &str) -> Result<bool> {
    project::validate_name(target)?;
    let dir = project::project_dir(root, target)?;
    let (file, text) = read_project_file(&dir)?;
    let cst = parse(&file, &text)?;

    let prop = mount_node(&cst, &source.mount).and_then(|node| node.get(&source.name));
    let Some(prop) = prop.filter(|prop| path_of(&prop.value()).is_some_and(|path| path == link_path)) else {
        log::info(format!("\"{}/{}\" isn't linked into \"{target}\"", source.project, source.rel));
        return Ok(false);
    };

    prop.remove();
    fs::write(&file, cst.to_string()).with_context(|| format!("failed to write \"{}\"", file.display()))?;
    Ok(true)
}

/// Works out where `arg` (`<project>/<path inside it>`) sits in its project's Rojo tree
fn resolve_source(root: &Path, arg: &str, name: Option<&str>, must_exist: bool) -> Result<Source> {
    if let Some(name) = name {
        let valid = !name.is_empty() && !name.starts_with('$') && !name.contains(['/', '\\', '"']);
        if !valid {
            bail!("\"{name}\" isn't a valid name for the link");
        }
    }

    let mut parts = Vec::new();
    for component in Path::new(arg).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => {}
            _ => bail!("\"{arg}\" should look like <project>/<path inside it>, e.g. tennis/client/services/Music.luau"),
        }
    }
    if parts.len() < 2 {
        bail!("\"{arg}\" should look like <project>/<path inside it>, e.g. tennis/client/services/Music.luau");
    }

    let project_name = parts.remove(0);
    project::validate_name(&project_name)?;
    let dir = project::project_dir(root, &project_name)?;

    let parts = match on_disk_casing(&dir, &parts) {
        Some(parts) => parts,
        None if must_exist => bail!("\"{}\" doesn't exist in \"{project_name}\"", parts.join("/")),
        None => parts,
    };
    let rel = parts.join("/");

    let (file, text) = read_project_file(&dir)?;
    let cst = parse(&file, &text)?;
    let tree = cst
        .object_value()
        .and_then(|object| object.object_value("tree"))
        .with_context(|| format!("\"{}\" has no tree", file.display()))?;

    let mut mounts = Vec::new();
    collect_mounts(&tree, &mut Vec::new(), &mut mounts);

    // the deepest folder the tree maps that holds the file, ignoring other projects like common
    let inside = |path: &String| rel.len() > path.len() && rel.starts_with(path.as_str()) && rel.as_bytes()[path.len()] == b'/';
    let (mount, mount_dir) = mounts
        .iter()
        .find(|(_, path)| *path == rel)
        .cloned()
        .or_else(|| mounts.iter().filter(|(_, path)| inside(path)).max_by_key(|(_, path)| path.len()).cloned())
        .with_context(|| {
            format!("\"{rel}\" isn't inside a folder that {PROJECT_FILE} maps, so there's nowhere to link it")
        })?;

    // a mapped folder itself goes under a name inside the same node in the target
    let whole_folder = mount_dir == rel;

    // a file inside the mount's folder, not a deeper one, since that folder has no node to add to
    if !whole_folder && rel[mount_dir.len() + 1..].contains('/') {
        bail!("\"{rel}\" is nested inside \"{mount_dir}\". Link the file or folder directly inside it instead.");
    }

    let name = match name {
        Some(name) => name.to_string(),
        None if whole_folder => project_name.clone(),
        None => instance_name(parts.last().expect("checked above"), dir.join(&rel).is_dir()),
    };
    Ok(Source { project: project_name, rel, mount, mount_dir, name })
}

/// Every node with a `$path` into this project, as (keys from the root, folder in the project)
fn collect_mounts(object: &CstObject, keys: &mut Vec<String>, out: &mut Vec<(Vec<String>, String)>) {
    for prop in object.properties() {
        let Some(name) = prop.name().and_then(|name| name.decoded_value().ok()) else { continue };
        if name.starts_with('$') {
            continue;
        }
        let Some(child) = prop.value().and_then(|value| value.as_object()) else { continue };

        keys.push(name);
        if let Some(path) = path_of(&prop.value()).and_then(|path| inside_project(&path)) {
            out.push((keys.clone(), path));
        }
        collect_mounts(&child, keys, out);
        keys.pop();
    }
}

/// The `$path` of a node, as written
fn path_of(value: &Option<CstNode>) -> Option<String> {
    value.as_ref()?.as_object()?.get("$path")?.value()?.as_string_lit()?.decoded_value().ok()
}

/// `./client/services` -> `client/services`, or None when it points outside the project
fn inside_project(path: &str) -> Option<String> {
    let mut parts = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_str()?),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

fn mount_node(cst: &CstRootNode, keys: &[String]) -> Option<CstObject> {
    let mut node = cst.object_value()?.object_value("tree")?;
    for key in keys {
        node = node.object_value(key)?;
    }
    Some(node)
}

fn read_project_file(dir: &Path) -> Result<(PathBuf, String)> {
    let file = dir.join(PROJECT_FILE);
    let text = fs::read_to_string(&file).with_context(|| format!("couldn't read \"{}\"", file.display()))?;
    Ok((file, text))
}

fn parse(file: &Path, text: &str) -> Result<CstRootNode> {
    CstRootNode::parse(text, &ParseOptions::default())
        .map_err(|err| anyhow::anyhow!("\"{}\" isn't valid: {err}", file.display()))
}

/// Matches each part against what is on disk ignoring case, so `tennis/Client/x.luau` links as typed on disk
fn on_disk_casing(dir: &Path, parts: &[String]) -> Option<Vec<String>> {
    let mut current = dir.to_path_buf();
    let mut found = Vec::new();

    for part in parts {
        let name = fs::read_dir(&current)
            .ok()?
            .flatten()
            .filter_map(|entry| entry.file_name().into_string().ok())
            .find(|name| name == part)
            .or_else(|| {
                fs::read_dir(&current)
                    .ok()?
                    .flatten()
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .find(|name| name.eq_ignore_ascii_case(part))
            })?;
        current.push(&name);
        found.push(name);
    }

    Some(found)
}

/// `MusicService.server.luau` -> `MusicService`, folders keep their name
fn instance_name(file_name: &str, is_dir: bool) -> String {
    if is_dir {
        return file_name.to_string();
    }

    SUFFIXES
        .iter()
        .find_map(|suffix| file_name.strip_suffix(suffix))
        .filter(|name| !name.is_empty())
        .unwrap_or(file_name)
        .to_string()
}

fn folder_has(dir: &Path, name: &str) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };

    entries.flatten().any(|entry| {
        entry
            .file_name()
            .to_str()
            .is_some_and(|file| instance_name(file, entry.path().is_dir()) == name)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROJECT: &str = r#"{
  "name": "boxing",
  "tree": {
    "$className": "DataModel",
    "ReplicatedStorage": {
      "Client": {
        "Services": {
          "$path": "./client/services",
          "common": { "$path": "../common/client/services" }
        }
      }
    }
  }
}"#;

    #[test]
    fn names_follow_rojo() {
        assert_eq!(instance_name("MusicService.luau", false), "MusicService");
        assert_eq!(instance_name("Main.client.luau", false), "Main");
        assert_eq!(instance_name("Part.model.json", false), "Part");
        assert_eq!(instance_name("music", true), "music");
        assert_eq!(instance_name(".luau", false), ".luau");
    }

    #[test]
    fn mounts_skip_other_projects() {
        let cst = CstRootNode::parse(PROJECT, &ParseOptions::default()).unwrap();
        let tree = cst.object_value().unwrap().object_value("tree").unwrap();
        let mut mounts = Vec::new();
        collect_mounts(&tree, &mut Vec::new(), &mut mounts);

        assert_eq!(
            mounts,
            vec![(vec!["ReplicatedStorage".into(), "Client".into(), "Services".into()], "client/services".to_string())]
        );
    }

    #[test]
    fn adds_and_removes_an_entry_in_place() {
        let cst = CstRootNode::parse(PROJECT, &ParseOptions::default()).unwrap();
        let keys: Vec<String> = ["ReplicatedStorage", "Client", "Services"].map(String::from).to_vec();
        let node = mount_node(&cst, &keys).unwrap();

        node.append(
            "Music",
            CstInputValue::Object(vec![("$path".to_string(), "../tennis/client/services/Music.luau".to_string().into())]),
        );
        let text = cst.to_string();
        assert!(text.contains("\"Music\""));
        assert!(text.contains("\"name\": \"boxing\""));

        let again = CstRootNode::parse(&text, &ParseOptions::default()).unwrap();
        let prop = mount_node(&again, &keys).unwrap().get("Music").unwrap();
        assert_eq!(path_of(&prop.value()).as_deref(), Some("../tennis/client/services/Music.luau"));
        prop.remove();
        assert!(!again.to_string().contains("Music"));
    }

    #[test]
    fn inside_project_rejects_outside_paths() {
        assert_eq!(inside_project("./client/services").as_deref(), Some("client/services"));
        assert_eq!(inside_project("../common/ui"), None);
    }
}
