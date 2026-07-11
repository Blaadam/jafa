# Jafa

This repository was created with [Jafa](https://jafa.adwo.dev) - A CLI tool making Roblox repositories - simpler.

## Getting Started

For Luau linting in VS Code, open the `jafa.code-workspace` or open the specific project folder, such as `projects/project-name`, so Luau LSP can watch that project's `sourcemap.json`.

To build the place, use:

```sh
jafa build <project-name>
```

Next, open the project in Roblox Studio with:

```sh
jafa open <project-name>
```

Finally, serve the project with the Rojo server:

```sh
jafa serve <project-name>
```

Or, you can do this all in one with

```sh
jafa dev <project-name>
```

For more help, check out [the Jafa repository](https://github.com/Blaadam/jafa)
