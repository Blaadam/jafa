import { resolve } from "node:path";
import shell from "shelljs";
import process from "node:process";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { log } from "../helpers/log.js";

const execute = (projectPath: string): Promise<string> => exec(`rojo serve ${escape(projectPath)}`);

export async function ServeProject(
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

    log.info(`Serving project "${projectName}"...`);
    log.verbose(`Executing "rojo serve" in "${projectDest}"...`);
    const buildOutput = await execute(resolve(projectDest, "default.project.json"));
    
    if (buildOutput) {
        log.info(`Serve output: ${buildOutput}`);
    }

    return projectDest;
}
