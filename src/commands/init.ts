import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import type { CommandDefinition } from "./types.js";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";

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

    shell.cd(cwd);
    console.log(`Initializing jafa in ${cwd}`);

    // Copy the scaffolding files to the target directory
    const scaffoldingRoot = resolve(__dirname, "../../scaffolding");

    const workspaceDir = resolve(scaffoldingRoot, "workspace");
    const workspaceEntries = readdirSync(workspaceDir).map((entry) => resolve(workspaceDir, entry));
    shell.cp("-R", workspaceEntries, ".");

    // Setup git
    if (shell.test("-d", ".git")) {
      console.log("Git repository already exists, skipping.");
    } else {
      console.log("Initializing git repository...");
      shell.exec("git init");
    }

    // Install dependencies
    console.log("Installing dependencies...");
    shell.exec("rokit install");

    // Creating a starter project
    const projectDest = resolve(cwd, "projects", "project-name");
    shell.mkdir("-p", dirname(projectDest));
    shell.cp("-R", resolve(scaffoldingRoot, "project"), projectDest);


    //shell.cp("-R", resolve(__dirname, "../../scaffolding/project"), ".");
    console.log(`Your jafa project has been initialized in ${cwd}.`);
  },
};
