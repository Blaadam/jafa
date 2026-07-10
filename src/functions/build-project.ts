import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { ReadJsonFile } from "../helpers/read-json.js";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const execute = (projectPath: string, buildPath: string): Promise<string> => exec(`rojo build -o ${escape(buildPath)} ${escape(projectPath)}`);

export async function BuildProject(
    projectName: string,
    target: string,
): Promise<string> {
    const cwd = resolve(process.cwd(), target);

    // Move to the target directory
    const projectsDir = resolve(cwd, "projects");
    const projectDest = resolve(projectsDir, projectName);

    if (!shell.test("-d", projectDest)) {
        throw new Error(`"${projectName}" doesn't exist in "${projectsDir}". Make sure the project exists.`);
        return "";
    }

    shell.cd(projectDest);

    // Create the build directory if it doesn't exist
    const buildDir = resolve(cwd, "artifacts", projectName);
    shell.mkdir("-p", buildDir);

    console.log(`Building project "${projectName}"...`);
    const buildOutput = await execute(resolve(projectDest, "default.project.json"), resolve(buildDir, "build.rbxl"));
    
    if (buildOutput) {
        console.log(`Build output:\n${buildOutput}`);
    }

    return projectDest;
}
