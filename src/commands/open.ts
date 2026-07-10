import { resolve, dirname } from "node:path";
import type { CommandDefinition } from "./types.js";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { CreateProject } from "../functions/create-project.js";
import { BuildProject } from "../functions/build-project.js";
import { OpenProject } from "../functions/open-project.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const openCommand: CommandDefinition = {
  name: "open",
  description: "Open a project within the current jafa workspace",
  usage: "open <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "open": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"open" requires a project name, e.g. "jafa open my-project"`);
    }

    OpenProject(name, target || ".");
  },
};
