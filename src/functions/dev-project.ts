import { log } from "../helpers/log.js";
import { BuildProject } from "./build-project.js";
import { OpenProject } from "./open-project.js";
import { ServeProject } from "./serve-project.js";
import shelly from "shelljs";

export async function DevProject(
	projectName: string,
	target: string,
): Promise<string> {
	const dir = process.cwd();
	log.info(`Starting development for project: ${projectName}`);

	shelly.cd(dir);
	log.trace(`cwd: ${process.cwd()}, target: ${target}`);
	log.verbose(`Building project`);
	await BuildProject(projectName, target);

	shelly.cd(dir);
	log.trace(`cwd: ${process.cwd()}, target: ${target}`);
	log.verbose(`Opening project`);
	await OpenProject(projectName, target);

	shelly.cd(dir);
	log.trace(`cwd: ${process.cwd()}, target: ${target}`);
	return ServeProject(projectName, target);
}
