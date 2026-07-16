<p align="center">
  <img src="website/static/img/jafa_logo.svg" width="150" alt="Jafa logo" />
</p>

<h1 align="center">Jafa</h1>

<p align="center">
  A CLI tool making Roblox repositories – simpler
</p>

---

Jafa is a CLI tool for managing Roblox monorepos. It scaffolds a shared workspace integrating popular tools (Rojo, Wally, Rokit, Selene, StyLua) and lets you spin up new projects inside it with a consistent folder structure, so multiple Roblox experiences can live side by side in one repository - or share resources!

## Installation

```sh
npm install -g @blaadam/jafa
```

> **NOTE**: Until the package is uploaded to npm, please use:

```sh
npm run build
npm link
```

## Usage

```sh
jafa <command> [options]
```

### Commands

| Command | Description |
| --- | --- |
| `init [target-directory]` | Create a new Jafa workspace in the given directory (defaults to the current directory). |
| `new <project-name> [target-directory]` | Create a new project inside an existing Jafa workspace. |
| `build <project-name>\|-a\|--all [target-directory]` | Build a project (or every project) with Rojo, writing to `artifacts/`. |
| `open <project-name>\|-a\|--all [target-directory]` | Open a project's built `.rbxl` file (or every built project) in Roblox Studio. |
| `serve <project-name> [target-directory]` | Serve a project for live-sync with Roblox Studio via `rojo serve`. |
| `dev <project-name> [target-directory]` | Build, open, and serve a project, in order. |
| `deploy <project-name>\|-a\|--all [target-directory] [-e env] [-g groupid] [-p payments] [-n name]` | Build a project and deploy it to Roblox using Mantle. |
| `sourcemap <project-name>\|-a\|--all [target-directory]` | Generate a `sourcemap.json` for a project (or every project). |
| `pull <project-name>\|-a\|--all [target-directory]` | Pull published Roblox assets (terrain, maps, lighting, etc.) into a project (or every project), as described by its `.jafa` file. |
| `help` | Show help information. |
| `version` | Show the CLI version. |

`build`, `open`, `deploy`, `sourcemap`, and `pull` accept `-a`/`--all` to run against every project under `projects/` instead of a single named one.

### Examples

```sh
jafa init .
jafa new my-experience
jafa dev my-experience
jafa build --all
jafa deploy my-experience -e prod -g 35171099 -p group
jafa pull my-experience
jafa --help
jafa --version
```

## What `init` creates

Running `jafa init` scaffolds a workspace with:

- `rokit.toml`, `wally/wally.toml` for tool and package management
- `jafa.code-workspace`, `.vscode/`, `.gitignore`, `README.md`
- `asset-updater.luau`, the Lune script `jafa pull` uses to pull published
  Roblox assets into a project
- A git repository (if one doesn't already exist)
- Tools installed via `rokit install` (Rojo, Wally, StyLua, Lune, Mantle, Selene, wally-package-types)
- Wally packages installed via `wally install --project-path ./wally`
- A `projects/` directory containing a starter project (see below — this is where per-project files like `default.project.json`, `selene.toml`, and `stylua.toml` live)

## What `new` creates

Running `jafa new <project-name>` adds a new project under `projects/<project-name>`,
scaffolded with client/server/shared/UI folder structure (services, controllers,
modules, databases, components, etc.), a `.jafa` asset manifest used by `jafa pull`,
and a Rojo project tree wired up to `ReplicatedStorage`, `ServerScriptService`,
and `Workspace`.

## Documentation

Full docs, including per-command reference and workspace/project concepts, are at [jafa.adwo.dev](https://jafa.adwo.dev), or run `npm run docs` to serve them locally from `website/`.

## Roadmap

- [x] `init` — create a Jafa workspace
- [x] `new` — create a new project (resource)
- [x] `build` — compile a project
- [x] `open` — open a project
- [x] `serve` — serve a project
- [x] `dev` — build, open, and serve a project (in order)
- [x] `deploy` — build and deploy a project to Roblox using Mantle
- [x] `sourcemap` — generate a sourcemap for a project
- [x] `pull` — pull published Roblox assets into a project

## Development

```sh
npm install
npm run build
```

This compiles TypeScript from `src/` to `dist/`, which is what the `jafa` bin points to.
