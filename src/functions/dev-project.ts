import { OpenProject } from "./open-project.js";
import { ServeProject } from "./serve-project.js";

export async function DevProject(
    projectName: string,
    target: string,
): Promise<string> {
    await OpenProject(projectName, target);
    return ServeProject(projectName, target);
}
