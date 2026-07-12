import type { CommandDefinition } from "./types.js";
import { BuildProject } from "../functions/build-project.js";
import { ListProjects } from "../helpers/list-projects.js";
import { log } from "../helpers/log.js";

export const buildCommand: CommandDefinition = {
  name: "build",
  description: "Build a project within the current jafa workspace",
  usage: "build <project-name> [target-directory] | build -a|--all [target-directory]",
  async run(args) {
    const all = args.includes("-a") || args.includes("--all");
    const unknownFlags = args.filter((arg) => arg.startsWith("-") && arg !== "-a" && arg !== "--all");
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "build": ${unknownFlags.join(", ")}`);
    }

    const positional = args.filter((arg) => !arg.startsWith("-"));

    if (all) {
      const target = positional[0] || ".";
      const names = ListProjects(target);
      if (names.length === 0) {
        throw new Error(`No projects found to build.`);
      }

      const failures: string[] = [];
      for (const name of names) {
        try {
          await BuildProject(name, target);
        } catch (err) {
          failures.push(name);
          log.error(`Failed to build "${name}": ${err instanceof Error ? err.message : String(err)}`);
        }
      }

      if (failures.length > 0) {
        throw new Error(`Failed to build: ${failures.join(", ")}`);
      }
      return;
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"build" requires a project name, e.g. "jafa build my-project", or use -a/--all to build all projects`);
    }

    await BuildProject(name, target || ".");
  },
};
