import { resolve, dirname } from "node:path";
import type { CommandDefinition } from "./types.js";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const newCommand: CommandDefinition = {
  name: "new",
  description: "Create a new project within the current jafa workspace",
  usage: "new <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "new": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"new" requires a project name, e.g. "jafa new my-project"`);
    }

    // Sets the specified target directory or defaults to the current working directory
    const workspaceDir = resolve(process.cwd(), target ?? ".");
    const scaffoldingRoot = resolve(__dirname, "../../scaffolding");
    const projectsDir = resolve(workspaceDir, "projects");
    const projectDest = resolve(projectsDir, name);

    // Check if being used in a jafa workspace
    if (!shell.test("-d", projectsDir)) {
      throw new Error(`"${projectsDir}" doesn't exist. Make sure "${workspaceDir}" is a jafa workspace.`);
    }
    // Check if the project exists encase of overwrite
    if (shell.test("-e", projectDest)) {
      throw new Error(`"${projectDest}" already exists`);
    }

    // Copy the project over to the workspace
    shell.cp("-R", resolve(scaffoldingRoot, "project"), projectDest);

    console.log(`Created new project "${name}" in ${projectDest}.`);
  },
};
