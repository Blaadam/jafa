import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export function CreateProject(
	projectName: string,
	target: string,
	checkExists: boolean = true,
): string {
	const cwd = resolve(process.cwd(), target);

	shell.cd(cwd);
	console.log(`Initializing jafa in ${cwd}`);

	// Copy the scaffolding files to the target directory
	const scaffoldingRoot = resolve(__dirname, "../../scaffolding");

	// Creating a starter project
	const projectsDir = resolve(cwd, "projects");
	const projectDest = resolve(projectsDir, projectName);

	if (checkExists) {
		// Check if being used in a jafa workspace
		if (!shell.test("-d", projectsDir)) {
			throw new Error(
				`"${projectsDir}" doesn't exist. Make sure "${cwd}" is a jafa workspace.`,
			);
			return "";
		}

		// Check if the project exists encase of overwrite
		if (shell.test("-e", projectDest)) {
			throw new Error(`"${projectDest}" already exists`);
			return "";
		}
	}

	shell.mkdir("-p", dirname(projectDest));
	shell.cp("-R", resolve(scaffoldingRoot, "project"), projectDest);

	//shell.cp("-R", resolve(__dirname, "../../scaffolding/project"), ".");
	console.log(`Your jafa project has been initialized in ${cwd}.`);

	return projectDest;
}
