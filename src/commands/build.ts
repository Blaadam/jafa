import type { CommandDefinition } from "./types.js";
import { BuildProject } from "../functions/build-project.js";

export const buildCommand: CommandDefinition = {
  name: "build",
  description: "Build a project within the current jafa workspace",
  usage: "build <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "build": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"build" requires a project name, e.g. "jafa build my-project"`);
    }

    BuildProject(name, target || ".");
  },
};
