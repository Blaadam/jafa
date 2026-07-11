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
npm link jafa
```

## Usage

```sh
jafa <command> [options]
```

### Commands

| Command                                 | Description                                                                                                                                                                                                  |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `init [directory]`                      | Create a new Jafa workspace in the given directory (defaults to the current directory). Sets up Rojo/Wally/Rokit config, initializes git, installs tools via `rokit install`, and creates a starter project. |
| `new <project-name> [target-directory]` | Create a new project inside an existing Jafa workspace.                                                                                                                                                      |
| `help`                                  | Show help information.                                                                                                                                                                                       |
| `version`                               | Show the CLI version.                                                                                                                                                                                        |

### Examples

```sh
jafa init .
jafa new my-experience
jafa --help
jafa --version
```

## What `init` creates

Running `jafa init` scaffolds a workspace with:

- `default.project.json`, `rokit.toml`, `selene.toml`, `stylua.toml`
- `wally/wally.toml` for package management
- A git repository (if one doesn't already exist)
- Tools installed via `rokit install` (Rojo, Wally, StyLua, Lune, Mantle, Selene, wally-package-types)
- A `projects/` directory containing a starter project

## What `new` creates

Running `jafa new <project-name>` adds a new project under `projects/<project-name>`,
scaffolded with client/server/shared/UI folder structure (services, controllers,
modules, databases, components, etc.) and a Rojo project tree wired up to
`ReplicatedStorage`, `ServerScriptService`, and `Workspace`.

## Roadmap

- [x] `init` — create a Jafa workspace
- [x] `new` — create a new project (resource)
- [x] `build` — compile a project
- [x] `open` — open a project
- [x] `serve` — serve a project
- [x] `dev` — build, open, and serve a project (in order)

## Development

```sh
npm install
npm run build
```

This compiles TypeScript from `src/` to `dist/`, which is what the `jafa` bin points to.
