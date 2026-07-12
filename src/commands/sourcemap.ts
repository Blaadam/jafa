import type { CommandDefinition } from "./types.js";
import { SourceProject } from "../functions/source-project.js";
import { ListProjects } from "../helpers/list-projects.js";
import { log } from "../helpers/log.js";

export const sourcemapCommand: CommandDefinition = {
  name: "sourcemap",
  description: "Generate a sourcemap for a project within the current jafa workspace",
  usage: "sourcemap <project-name> [target-directory] | sourcemap -a|--all [target-directory]",
  async run(args) {
    const all = args.includes("-a") || args.includes("--all");
    const unknownFlags = args.filter((arg) => arg.startsWith("-") && arg !== "-a" && arg !== "--all");
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "sourcemap": ${unknownFlags.join(", ")}`);
    }

    const positional = args.filter((arg) => !arg.startsWith("-"));

    if (all) {
      const target = positional[0] || ".";
      const names = ListProjects(target);
      if (names.length === 0) {
        throw new Error(`No projects found to generate a sourcemap for.`);
      }

      const failures: string[] = [];
      for (const name of names) {
        try {
          await SourceProject(name, target);
        } catch (err) {
          failures.push(name);
          log.error(`Failed to generate sourcemap for "${name}": ${err instanceof Error ? err.message : String(err)}`);
        }
      }

      if (failures.length > 0) {
        throw new Error(`Failed to generate sourcemap for: ${failures.join(", ")}`);
      }
      return;
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"sourcemap" requires a project name, e.g. "jafa sourcemap my-project", or use -a/--all to generate sourcemaps for all projects`);
    }

    await SourceProject(name, target || ".");
  },
};
