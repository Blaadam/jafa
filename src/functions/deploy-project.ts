import { relative, resolve } from "node:path";
import process from "node:process";
import shell from "shelljs";
import { parse as parseYaml, stringify as stringifyYaml } from "yaml";
import exec from "../utilities/exec.js";
import escape from "../utilities/escape.js";
import { log } from "../helpers/log.js";
import { BuildProject } from "./build-project.js";

export interface DeployOptions {
    environment: string;
    groupId?: string;
    payments?: string;
    name?: string;
}

interface MantleEnvironment {
    label: string;
    targetAccess?: string;
    targetNamePrefix?: string;
    [key: string]: unknown;
}

const ENVIRONMENT_LABEL_PREFIXED_ENVIRONMENTS = ["dev", "stg"];

interface MantleConfig {
    owner?: { group: number } | "personal";
    payments?: string;
    target: {
        experience: {
            places: {
                start: {
                    file: string;
                    configuration?: { name?: string; [key: string]: unknown };
                    [key: string]: unknown;
                };
                [key: string]: unknown;
            };
            [key: string]: unknown;
        };
        [key: string]: unknown;
    };
    environments: MantleEnvironment[];
    [key: string]: unknown;
}

const execute = (mantleDir: string, environment: string): Promise<string> =>
    exec(`mantle deploy ${escape(mantleDir)} --environment ${escape(environment)}`);

export async function DeployProject(
    projectName: string,
    target: string,
    options: DeployOptions,
): Promise<string> {
    const cwd = resolve(process.cwd(), target);

    const projectsDir = resolve(cwd, "projects");
    const projectDest = resolve(projectsDir, projectName);

    if (!shell.test("-d", projectDest)) {
        log.error(`"${projectName}" doesn't exist in "${projectsDir}". Make sure the project exists.`);
        throw new Error(`"${projectName}" doesn't exist in "${projectsDir}". Make sure the project exists.`);
    }

    log.info(`Building project "${projectName}" before deploying...`);
    await BuildProject(projectName, target);

    const buildFile = resolve(cwd, "artifacts", projectName, "build.rbxl");
    if (!shell.test("-f", buildFile)) {
        log.error(`Build output "${buildFile}" is missing after building "${projectName}".`);
        throw new Error(`Build output "${buildFile}" is missing after building "${projectName}".`);
    }

    const mantleDir = resolve(cwd, "artifacts", projectName, options.environment);
    shell.mkdir("-p", mantleDir);

    const mantleYmlPath = resolve(mantleDir, "mantle.yml");
    const placeFile = relative(mantleDir, buildFile).split("\\").join("/");

    let config: MantleConfig;
    if (shell.test("-f", mantleYmlPath)) {
        log.verbose(`Updating existing mantle.yml at "${mantleYmlPath}"...`);
        config = parseYaml(shell.cat(mantleYmlPath).toString()) as MantleConfig;
    } else {
        log.verbose(`Creating new mantle.yml at "${mantleYmlPath}"...`);
        config = {
            target: { experience: { places: { start: { file: placeFile } } } },
            environments: [],
        };
    }

    config.target ??= { experience: { places: { start: { file: placeFile } } } };
    config.target.experience ??= { places: { start: { file: placeFile } } };
    config.target.experience.places ??= { start: { file: placeFile } };
    config.target.experience.places.start = {
        ...config.target.experience.places.start,
        file: placeFile,
    };

    if (options.groupId) {
        if (!/^\d+$/.test(options.groupId)) {
            throw new Error(`"--groupid/-g" must be a numeric Roblox group ID, got "${options.groupId}"`);
        }
        config.owner = { group: Number(options.groupId) };
    }

    if (options.payments) {
        config.payments = options.payments;
    }

    if (options.name) {
        config.target.experience.places.start.configuration = {
            ...config.target.experience.places.start.configuration,
            name: options.name,
        };
    }

    if (!config.owner && !options.groupId) {
        log.warn(`No group ID configured for "${projectName}"; resources will be created under the personal account. Pass --groupid/-g to deploy under a group instead.`);
    }

    config.environments ??= [];
    const existingEnvironment = config.environments.find((environment) => environment.label === options.environment);
    const usesEnvironmentLabelPrefix = ENVIRONMENT_LABEL_PREFIXED_ENVIRONMENTS.includes(options.environment);

    if (existingEnvironment) {
        if (usesEnvironmentLabelPrefix) {
            existingEnvironment.targetNamePrefix = "environmentLabel";
        }
    } else {
        const newEnvironment: MantleEnvironment = { label: options.environment, targetAccess: "private" };
        if (usesEnvironmentLabelPrefix) {
            newEnvironment.targetNamePrefix = "environmentLabel";
        }
        config.environments.push(newEnvironment);
    }

    shell.ShellString(stringifyYaml(config)).to(mantleYmlPath);

    log.info(`Deploying project "${projectName}" to environment "${options.environment}"...`);
    log.verbose(`Executing "mantle deploy" in "${mantleDir}"...`);
    const deployOutput = await execute(mantleDir, options.environment);

    if (deployOutput) {
        log.info(`Deploy output: ${deployOutput}`);
    }

    return mantleDir;
}
