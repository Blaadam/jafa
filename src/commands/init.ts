import { resolve, dirname } from "node:path";
import type { CommandDefinition } from "./types.js";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { CreateWorkspace } from "../functions/create-workspace.js";
import { log } from "../helpers/log.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const initCommand: CommandDefinition = {
  name: "init",
  description: "Create a new jafa project in the given directory (defaults to the current directory)",
  usage: "init [target-directory]",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown flags: ${unknownFlags.join(", ")}`);
    }

    // Sets the specified target directory or defaults to the current working directory
    const target = args.find((arg) => !arg.startsWith("-")) ?? ".";
    const cwd = resolve(process.cwd(), target);

    const workspace = CreateWorkspace(cwd);
    
    log.info(`Initialized jafa workspace in ${workspace}`);
  },
};
