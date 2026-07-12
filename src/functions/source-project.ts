import { resolve } from "node:path";
import shell from "shelljs";
import process from "node:process";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { log } from "../helpers/log.js";

export async function SourceProject(
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

    log.info(`Building sourcemap for "${projectName}"`)
    shell.exec(`rojo sourcemap -o ${escape(resolve(projectDest, "sourcemap.json"))}`);

    return projectDest;
}
