use anyhow::{Context, Result};
use serde_json::json;
use std::fs;
use std::path::Path;

use crate::cli::{ObscureArgs, ObscureLevel};
use crate::{exec, log, project};

pub fn run(args: &ObscureArgs) -> Result<()> {
    let (root, names) = project::select(
        args.project.name.as_deref(),
        args.project.target.as_deref(),
        args.project.all,
        "obscure",
    )?;

    project::run_each(&names, "obscure", |name| {
        obscure_project(&root, name, args.level)
    })
}

pub fn obscure_project(root: &Path, name: &str, level: ObscureLevel) -> Result<()> {
    let dir = project::project_dir(root, name)?;

    let build_dir = project::artifacts_dir(root, name);
    fs::create_dir_all(&build_dir)
        .with_context(|| format!("failed to create \"{}\"", build_dir.display()))?;

    // darklua reads a whole directory tree, so the project (and the shared `common`
    // project and `wally` packages it may reference with `../`) are copied out to a
    // scratch area first. The real project source on disk is never touched.
    let src = build_dir.join(".obscure-src");
    let out = build_dir.join(".obscure-out");
    for scratch in [&src, &out] {
        if scratch.exists() {
            fs::remove_dir_all(scratch)
                .with_context(|| format!("failed to clear \"{}\"", scratch.display()))?;
        }
    }

    log::verbose(format!("Staging a copy of \"{name}\" to obfuscate..."));
    copy_dir(&dir, &src.join("projects").join(name), true)?;

    let common_dir = project::projects_dir(root).join("common");
    let has_common = common_dir.is_dir();
    if has_common {
        copy_dir(&common_dir, &src.join("projects").join("common"), true)?;
    }

    let wally_dir = root.join("wally");
    if wally_dir.is_dir() {
        // dependencies aren't part of "your" code, so they're copied through as-is
        copy_dir(&wally_dir, &out.join("wally"), true)?;
    }

    let config_path = src.join(".darklua.json");
    fs::write(&config_path, darklua_config(level))
        .with_context(|| format!("failed to write \"{}\"", config_path.display()))?;

    log::info(format!("Obfuscating scripts in \"{name}\"..."));
    let result = obfuscate_and_build(root, name, &src, &out, &config_path, has_common, &build_dir);

    // the scratch dirs can hold a copy of your source, so both are cleaned up even if
    // darklua or rojo failed partway through, not just on the success path. A build
    // failure is reported over a cleanup failure, since it's the more useful error.
    let mut cleanup_err = None;
    for scratch in [&src, &out] {
        if let Err(err) = fs::remove_dir_all(scratch) {
            cleanup_err.get_or_insert_with(|| {
                anyhow::anyhow!(err).context(format!("failed to clean up \"{}\"", scratch.display()))
            });
        }
    }

    result?;
    match cleanup_err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn obfuscate_and_build(
    root: &Path,
    name: &str,
    src: &Path,
    out: &Path,
    config_path: &Path,
    has_common: bool,
    build_dir: &Path,
) -> Result<()> {
    let project_src = src.join("projects").join(name);
    let project_out = out.join("projects").join(name);
    run_darklua(root, &project_src, &project_out, config_path)?;
    // darklua only writes a file it recognizes a loader for, and only creates a
    // directory when it writes something into it. That drops things like a binary
    // `.rbxm` model, or a folder with no Lua files in it (e.g. `client/controllers`
    // before you've added a controller), from the output entirely — even though
    // default.project.json still has a `$path` pointing at them, which then makes rojo
    // fail to find it. Carrying over whatever's missing from the pre-darklua copy fixes
    // that without changing any project files, and without touching what darklua wrote.
    copy_dir(&project_src, &project_out, false)?;

    if has_common {
        let common_src = src.join("projects").join("common");
        let common_out = out.join("projects").join("common");
        run_darklua(root, &common_src, &common_out, config_path)?;
        copy_dir(&common_src, &common_out, false)?;
    }

    let staged_dir = out.join("projects").join(name);

    log::info(format!("Building obfuscated project \"{name}\"..."));
    let output = exec::run(
        exec::command("rojo", &staged_dir)
            .arg("build")
            .arg("-o")
            .arg(build_dir.join("build.rbxl"))
            .arg(staged_dir.join("default.project.json")),
    )?;

    if !output.is_empty() {
        log::info(format!("Build output: {output}"));
    }

    Ok(())
}

fn run_darklua(cwd: &Path, input: &Path, output: &Path, config: &Path) -> Result<()> {
    exec::run(
        exec::command("darklua", cwd)
            .arg("process")
            .arg(input)
            .arg(output)
            .arg("--config")
            .arg(config),
    )?;

    Ok(())
}

fn darklua_config(level: ObscureLevel) -> String {
    let mut rules = vec![json!("remove_comments"), json!("remove_spaces")];

    if level != ObscureLevel::Minify {
        rules.push(json!({
            // "$roblox" keeps Roblox globals (game, script, require, ...) untouched,
            // "$default" keeps everything else darklua treats as a global out of safety
            "rule": "rename_variables",
            "globals": ["$default", "$roblox"],
        }));
    }

    if level == ObscureLevel::Full {
        rules.push(json!("remove_types"));
        rules.push(json!("remove_unused_variable"));
        rules.push(json!("remove_unused_if_branch"));
        rules.push(json!("compute_expression"));
    }

    let config = json!({
        "generator": "dense",
        "rules": rules,
        // darklua only emits files it has a loader for, and by default a loader for
        // a non-Lua extension (e.g. "json") converts the file into a Lua module instead
        // of passing it through. Without this, `default.project.json`, `selene.toml`,
        // etc. would silently vanish from the obfuscated copy instead of reaching it
        // unchanged, which rojo then fails to build against.
        //
        // These patterns are deliberately non-overlapping with `.lua`/`.luau` (left on
        // darklua's default loader) so match order can't matter: `serde_json` sorts
        // object keys alphabetically when it serializes this, which would silently
        // break a broader catch-all pattern that depended on being listed before them.
        "loaders": {
            "**/*.json": "copy",
            "**/*.json5": "copy",
            "**/*.toml": "copy",
            "**/*.yaml": "copy",
            "**/*.yml": "copy",
            "**/*.txt": "copy",
        },
    });

    // unwrap: `config` is built from `json!` literals and strings, it always serializes
    serde_json::to_string_pretty(&config).unwrap()
}

/// Recreates every directory and copies through every file `out` is missing relative to
/// `src`. With `overwrite: true` (plain staging copies), every file is copied as-is.
/// With `overwrite: false`, an existing destination file is left alone — used to carry
/// over whatever darklua's output dropped without touching anything it did write (so
/// obfuscated scripts are left alone). darklua only writes a file it has a loader for,
/// and only creates a directory when it writes something into it, so this covers both
/// empty directories and files it has no loader for at all (e.g. a `.rbxm`/`.rbxmx`
/// model), which it otherwise drops silently.
fn copy_dir(src: &Path, out: &Path, overwrite: bool) -> Result<()> {
    fs::create_dir_all(out).with_context(|| format!("failed to create \"{}\"", out.display()))?;

    for entry in
        fs::read_dir(src).with_context(|| format!("failed to read \"{}\"", src.display()))?
    {
        let entry = entry?;
        let from = entry.path();
        let to = out.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to, overwrite)?;
        } else if overwrite || !to.exists() {
            fs::copy(&from, &to)
                .with_context(|| format!("failed to copy \"{}\"", from.display()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    fn rules(level: ObscureLevel) -> Vec<Value> {
        let config: Value = serde_json::from_str(&darklua_config(level)).unwrap();
        config["rules"].as_array().unwrap().clone()
    }

    fn rule_names(level: ObscureLevel) -> Vec<String> {
        rules(level)
            .iter()
            .map(|rule| match rule {
                Value::String(name) => name.clone(),
                Value::Object(obj) => obj["rule"].as_str().unwrap().to_string(),
                _ => panic!("unexpected rule shape: {rule:?}"),
            })
            .collect()
    }

    #[test]
    fn minify_only_strips_comments_and_spaces() {
        let names = rule_names(ObscureLevel::Minify);
        assert_eq!(names, ["remove_comments", "remove_spaces"]);
    }

    #[test]
    fn rename_adds_rename_variables_and_protects_roblox_globals() {
        let names = rule_names(ObscureLevel::Rename);
        assert_eq!(names, ["remove_comments", "remove_spaces", "rename_variables"]);

        let rename_rule = &rules(ObscureLevel::Rename)[2];
        let globals = rename_rule["globals"].as_array().unwrap();
        assert!(globals.contains(&Value::String("$roblox".into())));
    }

    #[test]
    fn full_adds_dead_code_and_type_stripping_on_top_of_rename() {
        let names = rule_names(ObscureLevel::Full);
        assert_eq!(
            names,
            [
                "remove_comments",
                "remove_spaces",
                "rename_variables",
                "remove_types",
                "remove_unused_variable",
                "remove_unused_if_branch",
                "compute_expression",
            ]
        );
    }

    #[test]
    fn every_level_minifies_with_the_dense_generator() {
        for level in [ObscureLevel::Minify, ObscureLevel::Rename, ObscureLevel::Full] {
            let config: Value = serde_json::from_str(&darklua_config(level)).unwrap();
            assert_eq!(config["generator"], "dense");
        }
    }

    #[test]
    fn every_level_copies_non_lua_files_through_unchanged() {
        // otherwise darklua silently drops default.project.json (and friends) from the
        // obfuscated copy instead of passing it through, and rojo build fails to find it
        for level in [ObscureLevel::Minify, ObscureLevel::Rename, ObscureLevel::Full] {
            let config: Value = serde_json::from_str(&darklua_config(level)).unwrap();
            let loaders = config["loaders"].as_object().unwrap();

            assert_eq!(loaders["**/*.json"], "copy");
            assert_eq!(loaders["**/*.toml"], "copy");

            // none of these patterns may overlap with .lua/.luau: serializing a JSON
            // object sorts its keys, so match order (which darklua relies on) can't be
            // trusted to keep a catch-all pattern from shadowing the luau loader
            for pattern in loaders.keys() {
                assert!(
                    !pattern.ends_with(".lua") && !pattern.ends_with(".luau") && pattern != "**/*",
                    "loader pattern \"{pattern}\" could shadow the default luau loader"
                );
            }
        }
    }

    #[test]
    fn copy_dir_overwrite_mirrors_nested_files_and_folders() {
        let base = std::env::temp_dir().join(format!("jafa-obscure-test-{}", std::process::id()));
        let src = base.join("src");
        let dst = base.join("dst");
        let _ = fs::remove_dir_all(&base);

        fs::create_dir_all(src.join("nested")).unwrap();
        fs::write(src.join("a.luau"), "return 1").unwrap();
        fs::write(src.join("nested").join("b.luau"), "return 2").unwrap();

        copy_dir(&src, &dst, true).unwrap();

        assert_eq!(fs::read_to_string(dst.join("a.luau")).unwrap(), "return 1");
        assert_eq!(
            fs::read_to_string(dst.join("nested").join("b.luau")).unwrap(),
            "return 2"
        );

        fs::remove_dir_all(&base).unwrap();
    }

    #[test]
    fn copy_dir_no_overwrite_recreates_empty_folders_and_copies_through_unhandled_files_only() {
        let base = std::env::temp_dir().join(format!("jafa-fill-missing-test-{}", std::process::id()));
        let src = base.join("src");
        let out = base.join("out");
        let _ = fs::remove_dir_all(&base);

        // "controllers" has no scripts in it, so darklua's output (unlike a plain copy)
        // never creates it; "modules" has a nested empty folder of its own, and a
        // binary asset darklua has no loader for (stand-in for a `.rbxm` model)
        fs::create_dir_all(src.join("controllers")).unwrap();
        fs::create_dir_all(src.join("modules").join("empty-nested")).unwrap();
        fs::write(src.join("modules").join("a.luau"), "return 1").unwrap();
        fs::write(src.join("Map.rbxm"), b"not real rbxm bytes").unwrap();

        // stand in for darklua's output: the folder with a script in it exists, the
        // script was obfuscated, and everything darklua has no loader for is missing
        fs::create_dir_all(out.join("modules")).unwrap();
        fs::write(out.join("modules").join("a.luau"), "return nil").unwrap();

        copy_dir(&src, &out, false).unwrap();

        assert!(out.join("controllers").is_dir());
        assert!(out.join("modules").join("empty-nested").is_dir());
        assert_eq!(fs::read(out.join("Map.rbxm")).unwrap(), b"not real rbxm bytes");
        // doesn't disturb (overwrite) a file darklua already wrote
        assert_eq!(fs::read_to_string(out.join("modules").join("a.luau")).unwrap(), "return nil");

        fs::remove_dir_all(&base).unwrap();
    }
}
