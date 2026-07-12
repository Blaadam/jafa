import type { CommandDefinition } from "./types.js";
import { SourceProject } from "../functions/source-project.js";

export const sourcemapCommand: CommandDefinition = {
  name: "sourcemap",
  description: "Generate a sourcemap for a project within the current jafa workspace",
  usage: "sourcemap <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "sourcemap": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"sourcemap" requires a project name, e.g. "jafa sourcemap my-project"`);
    }

    SourceProject(name, target || ".");
  },
};
