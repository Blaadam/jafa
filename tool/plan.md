# Plan

I want to rewrite this tool in rust. My aim is just to learn a new language, whilst also giving me something practical to achieve. This should in return also help improve some of the dependency concerns for Roblox Developers who don't already use Web Development tools like Node.js

# Core Commands I want to integrate:

1. `init` -> scaffolds the new repo from `/scaffolding/` and runs `git init`, `rokit install` -> would also be nice to have a "common" setting to create a project called "common" for shared modules and auto set them up.
2. `new` -> Creates a new project
3. `build` -> Runs `rojo build`
4. `sourcemap` -> Runs `rojo sourcemap`
5. `serve` -> Runs `rojo serve`
6. `open` -> Runs `start` / `open` / `xdg-open`
7. `dev` -> Builds, opens, and then serves
8. `deploy` -> Runs `mantle deploy` (I would like to be able to run custom deployments, replacing need for just in Super Sports project)
9. `pull` -> Runs `lune run asset-updater.luau`

# Proposed Structure

```
tool/
  Cargo.toml
  src/
    main.rs           # entry: parse, dispatch, print errors, exit code
    cli.rs            # clap definitions
    log.rs            # colored prefixed logger
    commands/         # init, new, build, open, serve, dev, deploy, sourcemap, pull
    project.rs        # resolve workspace/project paths, list_projects, "must exist" checks
    exec.rs           # run external tools
    workspace_file.rs # .jafa-workspace read/write (serde)
    mantle.rs         # mantle.yml model + merge logic
    scaffold.rs       # copy scaffolding, struct.json folder creation
```

## Crates to be used

* `clap` (derive) to replace `cli.ts` and the commands argument parsing.
* `anyhow` for error handling
* `serde`, `serde_json` to handle workspace files and project structuring
* `serde_yaml_ng` to handle mantle yaml's
* `fs_extra` to replace `shell.cp -R`
* `owo-colors` to replace `chalk` in the logger
* `include_dir` to embed `/scaffolding/` into the binary, so `jafa` is a single file with no Node.js or extra folders needed

# Decisions

* Binary is named `jafa` (`[[bin]]` in `Cargo.toml`), package is renamed from `tool`
* Fix the old quirks as I port instead of copying them:
  * Judge success by exit code, not by whether stderr has output (rojo and mantle write progress to stderr)
  * `serve` / `dev` stream output live
  * A missing project is always an error (non-zero exit)
* New features (`common` setting, custom deployments) come AFTER the port is at parity, so step 6 has a fair baseline

# Work Order

1. Scaffold and tooling (rename binary, add `tool/target` to `.gitignore`, clap skeleton, `log.rs`)
2. Write core helpers (`exec.rs` and `project.rs`)
3. Rebuild simple commands (`sourcemap`, `build`, `pull`, `serve`, `open`, `dev`) - all call other tools/commands
4. Rebuild `init` and `new` (embed scaffolding here)
5. Build `deploy` (same behaviour as the TS version for now)
6. Compare old outputs to new outputs
7. New features: `init` "common" setting, custom deployments (decide the config shape first)
8. Cutover (remove mono's src, update `package.json` and the website install docs)
9. Distribution without npm (GitHub Releases, `cargo install`, maybe Rokit)
