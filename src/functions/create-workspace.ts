import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { CreateProject } from "./create-project.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export function CreateWorkspace(target: string): string {
    const cwd = resolve(process.cwd(), target);

    shell.cd(cwd);
    console.log(`Initializing jafa in ${cwd}`);

    // Copy the scaffolding files to the target directory
    const scaffoldingRoot = resolve(__dirname, "../../scaffolding");

    const workspaceDir = resolve(scaffoldingRoot, "workspace");
    const workspaceEntries = readdirSync(workspaceDir).map((entry) =>
        resolve(workspaceDir, entry),
    );
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

    CreateProject("project-name", cwd, false);

    // Creating sub-folders

    return cwd;
}
