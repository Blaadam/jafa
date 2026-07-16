import { resolve, dirname } from "node:path";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { ReadJsonFile } from "../helpers/read-json.js";
import { log } from "../helpers/log.js";
import { AddToWorkspace } from "./workspace-file.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export function CreateProject(
	projectName: string,
	target: string,
	checkExists: boolean = true,
): string {
	const cwd = resolve(process.cwd(), target);

	shell.cd(cwd);
	log.info(`Initializing jafa in ${cwd}`);

	// Copy the scaffolding files to the target directory
	const scaffoldingRoot = resolve(__dirname, "../../scaffolding");

	// Creating a starter project
	const projectsDir = resolve(cwd, "projects");
	const projectDest = resolve(projectsDir, projectName);

	if (checkExists) {
		// Check if being used in a jafa workspace
		if (!shell.test("-d", projectsDir)) {
			log.error(`"${projectsDir}" doesn't exist. Make sure "${cwd}" is a jafa workspace.`);
			throw new Error(`"${projectsDir}" doesn't exist. Make sure "${cwd}" is a jafa workspace.`);
		}

		// Check if the project exists encase of overwrite
		if (shell.test("-e", projectDest)) {
			log.error(`"${projectDest}" already exists`);
			throw new Error(`"${projectDest}" already exists`);
		}
	}

	shell.mkdir("-p", dirname(projectDest));
	shell.cp("-R", resolve(scaffoldingRoot, "project"), projectDest);

	// Create sub-folders
	log.info(`Creating sub-folders for ${projectName}...`);
	const jsonStruct = ReadJsonFile(resolve(projectDest, "struct.json"));

	const createFolders = (basePath: string, struct: unknown): void => {
		if (!struct || typeof struct !== "object" || Array.isArray(struct)) {
			return;
		}

		for (const [folderName, nestedStruct] of Object.entries(
			struct as Record<string, unknown>,
		)) {
			const folderPath = resolve(basePath, folderName);
			log.verbose(`Creating folder: ${folderPath}`);
			shell.mkdir("-p", folderPath);
			createFolders(folderPath, nestedStruct);
		}
	};

	createFolders(projectDest, jsonStruct);

	shell.rm(resolve(projectDest, "struct.json"));

	log.verbose(`Adding project "${projectName}" to workspace file...`);
	AddToWorkspace(cwd, projectName);

	log.info(`Project "${projectName}" created successfully at "${projectDest}".`);

	log.info(`Your jafa project has been initialized in ${cwd}.`);

	return projectDest;
}
