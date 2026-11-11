# Jafa

A CLI tool making Roblox repositories simpler.

Jafa scaffolds a shared workspace for [Rojo](https://rojo.space/),
[Wally](https://wally.run/), [Rokit](https://github.com/rojo-rbx/rokit),
Selene and StyLua, then lets you add as many projects to it as you like with a
consistent folder structure. Several Roblox experiences can live side by side
in one repository, or share code through a common project. It's a single
binary and needs no Node.js.

## Install

```sh
cargo install jafa
```

You'll also need [Rokit](https://github.com/rojo-rbx/rokit) on your `PATH`, and
Git. Jafa runs `rokit install` for you, which installs Rojo, Wally, Lune,
Mantle and the rest.

## Quick start

```sh
jafa init my-game      # asks for a first project name and which tools to run
cd my-game
jafa new my-experience # add another project
jafa dev my-experience # build, open in Roblox Studio, and serve for live sync
```

To skip the questions: `jafa init my-game --yes --name lobby`.

## Commands

| Command | What it does |
| --- | --- |
| `init [dir]` | Create a workspace. Flags: `-y/--yes`, `-n/--name <project>`, `--common`. |
| `new <name> [dir]` | Add a project to the workspace. |
| `build <name>` | Build with Rojo into `artifacts/<name>/build.rbxl`. |
| `open <name>` | Open a built project in Roblox Studio. |
| `serve <name>` | Serve a project for live sync (`rojo serve`). |
| `dev <name>` | Build, open, then serve. |
| `sourcemap <name>` | Generate `sourcemap.json` for a project. |
| `pull <name>` | Pull published Roblox assets into a project, as listed in its `.jafa` file. |
| `deploy <name>` | Build and deploy one project as its own experience with Mantle. Flags: `-e`, `-g`, `-p`, `-n`. |
| `new-universe <name> [projects...]` | Create a starter Mantle config for an experience made of several places. |
| `deploy-universe <name>` | Build the projects a universe uses, then deploy it with Mantle. |

`build`, `open`, `deploy`, `sourcemap` and `pull` take `-a/--all` to run on every
project in `projects/`. Run `jafa <command> --help` for the details.

## Common project

A project named `common` holds code shared between your other projects. Create
it with `jafa init --common` or `jafa new common`. Every project made after it is
linked to it in its Rojo tree, so shared code appears beside each project's own.

## Universes

Roblox experiences can have several places. `jafa deploy-universe` builds the
projects whose builds a Mantle config points at, then runs `mantle deploy`. You
write the config, so Jafa never rewrites it. Start one with `jafa new-universe`.

## Documentation

Full docs are at [jafa.adwo.dev](https://jafa.adwo.dev).

## License

ISC
