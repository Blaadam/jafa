import type { CommandDefinition } from "./types.js";
import { OpenProject } from "../functions/open-project.js";
import { ListProjects } from "../helpers/list-projects.js";
import { log } from "../helpers/log.js";

export const openCommand: CommandDefinition = {
  name: "open",
  description: "Open a project within the current jafa workspace",
  usage: "open <project-name> [target-directory] | open -a|--all [target-directory]",
  async run(args) {
    const all = args.includes("-a") || args.includes("--all");
    const unknownFlags = args.filter((arg) => arg.startsWith("-") && arg !== "-a" && arg !== "--all");
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "open": ${unknownFlags.join(", ")}`);
    }

    const positional = args.filter((arg) => !arg.startsWith("-"));

    if (all) {
      const target = positional[0] || ".";
      const names = ListProjects(target);
      if (names.length === 0) {
        throw new Error(`No projects found to open.`);
      }

      const failures: string[] = [];
      for (const name of names) {
        try {
          await OpenProject(name, target);
        } catch (err) {
          failures.push(name);
          log.error(`Failed to open "${name}": ${err instanceof Error ? err.message : String(err)}`);
        }
      }

      if (failures.length > 0) {
        throw new Error(`Failed to open: ${failures.join(", ")}`);
      }
      return;
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"open" requires a project name, e.g. "jafa open my-project", or use -a/--all to open all projects`);
    }

    await OpenProject(name, target || ".");
  },
};
