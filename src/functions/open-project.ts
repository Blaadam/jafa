import { resolve, dirname } from "node:path";
import { readdirSync } from "node:fs";
import shell from "shelljs";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { ReadJsonFile } from "../helpers/read-json.js";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { execSync } from "node:child_process";
import os from "node:os";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

function start(buildPath: string): void {
    console.log(`[Gxxx] Opening build file: ${buildPath}`);
    
	switch (os.type()) {
		case "Windows_NT":
			execSync(`start "" "${buildPath}"`);
			break;
		case "Darwin":
			execSync(`open "${buildPath}"`);
			break;
		case "Linux":
			console.warn(
				`[Gxxx] Note that Linux support is limited both for Gaffer and Roblox.`,
			);
			execSync(`xdg-open "${buildPath}"`);
			break;
		default:
			throw new Error(
				`[Gxxx] Your operating system (${os.type()}) is not supported by this command.`,
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

	if (!shell.test("-d", buildDir)) {
		throw new Error(
			`"${projectName}" doesn't exist in "${artifactsDir}". Make sure the project exists.`,
		);
		return "";
	}


    // check if file exists
    const buildFile = resolve(buildDir, "build.rbxl");
    if (!shell.test("-f", buildFile)) {
        throw new Error(`Build file "${buildFile}" does not exist. Make sure the build was successful.`);
        return "";
    }

	start(buildFile);

	return "";
}
