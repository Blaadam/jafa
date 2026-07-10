import { resolve } from "node:path";
import shell from "shelljs";
import process from "node:process";
import { execSync } from "node:child_process";
import os from "node:os";
import { log } from "../helpers/log.js";

function start(buildPath: string): void {
	log.info(`Opening build file: ${buildPath}`);

	switch (os.type()) {
		case "Windows_NT":
			execSync(`start "" "${buildPath}"`);
			break;
		case "Darwin":
			execSync(`open "${buildPath}"`);
			break;
		case "Linux":
			log.warn(
				`Note that Linux support is limited both for Gaffer and Roblox.`,
			);
			execSync(`xdg-open "${buildPath}"`);
			break;
		default:
			log.error(
				`Your operating system (${os.type()}) is not supported by this command.`,
			);
			throw new Error(
				`Your operating system not supported.`,
			);
	}
}

export async function OpenProject(
	projectName: string,
	target: string,
): Promise<string> {
	const cwd = resolve(process.cwd(), target);

	const artifactsDir = resolve(cwd, "artifacts");
	const buildDir = resolve(artifactsDir, projectName);

	log.verbose(`Checking if build directory exists for ${projectName}...`);
	if (!shell.test("-d", buildDir)) {
		log.error(
			`"${projectName}" doesn't exist in "${artifactsDir}". Make sure the project exists.`,
		);
		throw new Error(
			`"${projectName}" doesn't exist in "${artifactsDir}". Make sure the project exists.`,
		);
	}

	// check if file exists
	const buildFile = resolve(buildDir, "build.rbxl");
	if (!shell.test("-f", buildFile)) {
		log.error(
			`Build file "${buildFile}" does not exist. Make sure the build was successful.`,
		);
		throw new Error(
			`Build file "${buildFile}" does not exist. Make sure the build was successful.`,
		);
	}

	log.verbose(`Opening build file: ${buildFile}`);
	start(buildFile);

	return "";
}
