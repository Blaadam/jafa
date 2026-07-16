import { resolve } from "node:path";
import shell from "shelljs";
import process from "node:process";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { log } from "../helpers/log.js";

const execute = (cwd: string, projectName: string): Promise<string> =>
    exec(`lune run asset-updater.luau ${escape(projectName)}`, true, cwd);

export async function PullProject(
    projectName: string,
    target: string,
): Promise<string> {
    const cwd = resolve(process.cwd(), target);

    const projectsDir = resolve(cwd, "projects");
    const projectDest = resolve(projectsDir, projectName);

    if (!shell.test("-d", projectDest)) {
        log.error(`"${projectName}" doesn't exist in "${projectsDir}". Make sure the project exists.`);
        return "";
    }

    const scriptPath = resolve(cwd, "asset-updater.luau");
    if (!shell.test("-f", scriptPath)) {
        log.error(`"asset-updater.luau" doesn't exist in "${cwd}". Make sure you're in a jafa workspace.`);
        return "";
    }

    log.info(`Pulling assets for project "${projectName}"...`);
    const pullOutput = await execute(cwd, projectName);

    if (pullOutput) {
        log.info(`Pull output: ${pullOutput}`);
    }

    return projectDest;
}
