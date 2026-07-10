import { resolve } from "node:path";
import shell from "shelljs";
import process from "node:process";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { log } from "../helpers/log.js";

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
        log.error(`"${projectName}" doesn't exist in "${projectsDir}". Make sure the project exists.`);
        return "";
    }

    shell.cd(projectDest);

    // Create the build directory if it doesn't exist
    log.verbose(`Ensuring build directory exists for ${projectName}...`);
    const buildDir = resolve(cwd, "artifacts", projectName);
    shell.mkdir("-p", buildDir);

    log.info(`Building project "${projectName}"...`);
    const buildOutput = await execute(resolve(projectDest, "default.project.json"), resolve(buildDir, "build.rbxl"));
    
    if (buildOutput) {
        log.info(`Build output: ${buildOutput}`);
    }

    return projectDest;
}
