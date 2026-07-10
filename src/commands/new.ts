import { dirname } from "node:path";
import type { CommandDefinition } from "./types.js";
import { fileURLToPath } from "node:url";
import { CreateProject } from "../functions/create-project.js";
import { log } from "../helpers/log.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const newCommand: CommandDefinition = {
  name: "new",
  description: "Create a new project within the current jafa workspace",
  usage: "new <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      log.error(`Unknown option(s) for "new": ${unknownFlags.join(", ")}`);
      return;
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      log.error(`"new" requires a project name, e.g. "jafa new my-project"`);
      return;
    }

    const dir = CreateProject(name, target || ".");
    log.info(`Created new project "${name}" in ${dir}`);
  },
};
