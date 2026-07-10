import type { CommandDefinition } from "./types.js";
import { ServeProject } from "../functions/serve-project.js";

export const serveCommand: CommandDefinition = {
  name: "serve",
  description: "Serve a project within the current jafa workspace",
  usage: "serve <project-name> [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "serve": ${unknownFlags.join(", ")}`);
    }

    const [name, target] = args.filter((arg) => !arg.startsWith("-"));
    if (!name) {
      throw new Error(`"serve" requires a project name, e.g. "jafa serve my-project"`);
    }

    ServeProject(name, target || ".");
  },
};
