import type { CommandDefinition } from "./types.js";
import { DevProject } from "../functions/dev-project.js";

export const devCommand: CommandDefinition = {
  name: "dev",
  description: "Open and serve a project at the same time within the current jafa workspace",
  usage: "dev <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "dev": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"dev" requires a project name, e.g. "jafa dev my-project"`);
    }

    DevProject(name, target || ".");
  },
};
