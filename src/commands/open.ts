import type { CommandDefinition } from "./types.js";
import { OpenProject } from "../functions/open-project.js";

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
