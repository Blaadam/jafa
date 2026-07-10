import { dirname } from "node:path";
import type { CommandDefinition } from "./types.js";
import { fileURLToPath } from "node:url";
import { CreateProject } from "../functions/create-project.js";

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

    const dir = CreateProject(name, target || ".");
    console.log(`Created new project "${name}" in ${dir}`);
  },
};
