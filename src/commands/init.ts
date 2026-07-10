import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import type { CommandDefinition } from "./types.js";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { CreateProject } from "../functions/create-project.js";
import { CreateWorkspace } from "../functions/create-workspace.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export const initCommand: CommandDefinition = {
  name: "init",
  description: "Create a new jafa project in the given directory (defaults to the current directory)",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "init": ${unknownFlags.join(", ")}`);
    }

    // Sets the specified target directory or defaults to the current working directory
    const target = args.find((arg) => !arg.startsWith("-")) ?? ".";
    const cwd = resolve(process.cwd(), target);

    const workspace = CreateWorkspace(cwd);
    console.log(`Initialized jafa workspace in ${workspace}`);
  },
};
