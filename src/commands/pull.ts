import type { CommandDefinition } from "./types.js";
import { PullProject } from "../functions/pull-project.js";
import { ListProjects } from "../helpers/list-projects.js";
import { log } from "../helpers/log.js";

export const pullCommand: CommandDefinition = {
  name: "pull",
  description: "Pull updates of roblox assets for a project or all projects in a directory",
  usage: "pull <project-name> [target-directory] | pull -a|--all [target-directory]",
  async run(args) {
    const all = args.includes("-a") || args.includes("--all");
    const unknownFlags = args.filter((arg) => arg.startsWith("-") && arg !== "-a" && arg !== "--all");
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "pull": ${unknownFlags.join(", ")}`);
    }

    const positional = args.filter((arg) => !arg.startsWith("-"));

    if (all) {
      const target = positional[0] || ".";
      const names = ListProjects(target);
      if (names.length === 0) {
        throw new Error(`No projects found to pull.`);
      }

      const failures: string[] = [];
      for (const name of names) {
        try {
          await PullProject(name, target);
        } catch (err) {
          failures.push(name);
          log.error(`Failed to pull "${name}": ${err instanceof Error ? err.message : String(err)}`);
        }
      }

      if (failures.length > 0) {
        throw new Error(`Failed to pull: ${failures.join(", ")}`);
      }
      return;
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"pull" requires a project name, e.g. "jafa pull my-project", or use -a/--all to pull all projects`);
    }

    await PullProject(name, target || ".");
  },
};
