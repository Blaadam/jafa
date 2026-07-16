import { resolve, dirname } from "node:path";
import shell from "shelljs";
import { log } from "../helpers/log.js";
import { ReadJsonFile } from "../helpers/read-json.js";

export interface WorkspaceProject {
	name: string;
	path: string;
}

export interface WorkspaceFile {
	created: string;
	projectsPath: string;
	projects: WorkspaceProject[];
}

export function CreateWorkspaceFile(cwd: string): WorkspaceFile {
    shell.cd(cwd);

	log.verbose("Creating jafa-workspace file...");
	const jafaWorkspaceFile = resolve(cwd, ".vscode/.jafa-workspace");
	const jafaWorkspaceContents: WorkspaceFile = {
		created: new Date().toISOString(),
		projectsPath: "jafa-workspace://projects",
		projects: [],
	};

	shell.touch(jafaWorkspaceFile);
	shell.echo(JSON.stringify(jafaWorkspaceContents)).to(jafaWorkspaceFile);

	return jafaWorkspaceContents;
}

export function AddToWorkspace(cwd: string, projectName: string): void {
	const workspaceFile = resolve(cwd, ".vscode/.jafa-workspace");
	if (!shell.test("-e", workspaceFile)) {
		log.error(`Workspace file "${workspaceFile}" does not exist.`);
		throw new Error(`Workspace file "${workspaceFile}" does not exist.`);
	}

	const workspaceData: WorkspaceFile = ReadJsonFile(workspaceFile) || CreateWorkspaceFile(cwd);
	if (!workspaceData.projects) {
		workspaceData.projects = [];
	}

	const projectPath = `projects://${projectName}`;

	const existingProject = workspaceData.projects.find((project) => project.name === projectName);
	if (existingProject) {
		log.verbose(`Updating existing project "${projectName}" in workspace file...`);
		existingProject.path = projectPath;
	} else {
		log.verbose(`Adding project "${projectName}" to workspace file...`);
		workspaceData.projects.push({ name: projectName, path: projectPath });
	}

	shell.echo(JSON.stringify(workspaceData)).to(workspaceFile);
}
