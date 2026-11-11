use anyhow::{Context, Result};
use jsonc_parser::ParseOptions;
use jsonc_parser::cst::{CstInputValue, CstNode, CstRootNode};
use std::fs;
use std::path::Path;

use crate::log;

const FILE: &str = "jafa.code-workspace";

pub fn add_project(root: &Path, name: &str) -> Result<()> {
    edit(root, |text| {
        let updated = insert_folder(text, name)?;
        log::verbose(match updated {
            Some(_) => format!("Adding \"{name}\" to {FILE}..."),
            None => format!("\"{name}\" is already in {FILE}."),
        });
        Ok(updated)
    })
}

/// The scaffolding lists a placeholder project, so point it at the real first project instead
pub fn rename_placeholder(root: &Path, name: &str) -> Result<()> {
    edit(root, |text| {
        Ok(Some(
            text.replace("projects/project-name", &format!("projects/{name}"))
                .replace("\"Project-Name\"", &format!("\"{}\"", display_name(name))),
        ))
    })
}

/// Runs `change` on the file's text and saves the result, if there is a file and a result
fn edit(root: &Path, change: impl FnOnce(&str) -> Result<Option<String>>) -> Result<()> {
    let file = root.join(FILE);
    if !file.is_file() {
        log::verbose(format!("No {FILE} found, skipping."));
        return Ok(());
    }

    if let Some(updated) = change(&fs::read_to_string(&file)?)? {
        fs::write(&file, updated)?;
    }

    Ok(())
}

/// `project-name` -> `Project-Name`
fn display_name(name: &str) -> String {
    name.split('-')
        .map(|part| {
            let (first, rest) = part.split_at(part.len().min(1));
            format!("{}{rest}", first.to_uppercase())
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn folder_path(folder: &CstNode) -> Option<String> {
    folder
        .as_object()?
        .get("path")?
        .value()?
        .as_string_lit()?
        .decoded_value()
        .ok()
}

/// Edits the file's text in place, since it has trailing commas and maybe comments that
/// serde_json can't read or keep. `Ok(None)` means the project is already listed.
fn insert_folder(text: &str, name: &str) -> Result<Option<String>> {
    let root = CstRootNode::parse(text, &ParseOptions::default())
        .map_err(|err| anyhow::anyhow!("{FILE} isn't valid: {err}"))?;
    let folders = root
        .object_value()
        .and_then(|object| object.array_value("folders"))
        .with_context(|| format!("couldn't find a \"folders\" list in {FILE}"))?;

    let path = format!("projects/{name}");
    let elements = folders.elements();
    let paths: Vec<Option<String>> = elements.iter().map(folder_path).collect();

    if paths.iter().flatten().any(|existing| *existing == path) {
        return Ok(None);
    }

    let folder = CstInputValue::Object(vec![
        ("name".to_string(), display_name(name).into()),
        ("path".to_string(), path.into()),
    ]);

    // sit next to the other projects when there are any
    match paths
        .iter()
        .rposition(|path| path.as_deref().is_some_and(|p| p.starts_with("projects/")))
    {
        Some(index) => folders.insert(index + 1, folder),
        None => folders.append(folder),
    };

    Ok(Some(root.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "{\n  \"folders\": [\n    {\n      \"name\": \"Repository\",\n      \"path\": \".\"\n    },\n    {\n      \"name\": \"Project-Name\",\n      \"path\": \"projects/project-name\"\n    },\n    {\n      \"name\": \"Wally\",\n      \"path\": \"wally\"\n    },\n  ],\n  \"settings\": { \"folders\": 1 },\n}\n";

    #[test]
    fn inserts_after_the_last_project() {
        let out = insert_folder(SAMPLE, "archery").unwrap().unwrap();
        let project = out.find("projects/project-name").unwrap();
        let new = out.find("projects/archery").unwrap();
        let wally = out.find("\"wally\"").unwrap();
        assert!(project < new && new < wally);
        assert!(out.contains("\"name\": \"Archery\""));
    }

    #[test]
    fn keeps_everything_else_untouched() {
        let out = insert_folder(SAMPLE, "archery").unwrap().unwrap();
        assert!(out.ends_with("  ],\n  \"settings\": { \"folders\": 1 },\n}\n"));
    }

    #[test]
    fn already_listed_is_left_alone() {
        assert!(insert_folder(SAMPLE, "project-name").unwrap().is_none());
    }

    #[test]
    fn follows_crlf_and_tabs() {
        let text = "{\r\n\t\"folders\": [\r\n\t\t{ \"name\": \"A\", \"path\": \"projects/a\" }\r\n\t]\r\n}";
        let out = insert_folder(text, "b").unwrap().unwrap();
        assert!(out.contains("\"path\": \"projects/b\""));
        assert!(!out.replace("\r\n", "").contains('\n'));
    }

    #[test]
    fn works_with_comments_and_an_empty_list() {
        let text = "{\n  // \"folders\": [ { \"path\": \"x\" } ]\n  \"folders\": [\n  ]\n}";
        let out = insert_folder(text, "a").unwrap().unwrap();
        assert!(out.contains("\"path\": \"projects/a\""));
        assert!(out.contains("// \"folders\""));
    }

    #[test]
    fn errors_without_a_folders_list() {
        assert!(insert_folder("{ \"settings\": {} }", "a").is_err());
    }
}
