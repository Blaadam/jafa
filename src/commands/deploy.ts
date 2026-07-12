import type { CommandDefinition } from "./types.js";
import { DeployProject } from "../functions/deploy-project.js";
import { ListProjects } from "../helpers/list-projects.js";
import { log } from "../helpers/log.js";

const FLAGS: Record<string, "environment" | "groupId" | "payments" | "name"> = {
  "--environment": "environment",
  "-e": "environment",
  "--groupid": "groupId",
  "-g": "groupId",
  "--payments": "payments",
  "-p": "payments",
  "--name": "name",
  "-n": "name",
};

const ALL_FLAGS = ["--all", "-a"];

export const deployCommand: CommandDefinition = {
  name: "deploy",
  description: "Build and deploy a project to Roblox using Mantle",
  usage: "deploy <project-name>|-a|--all [target-directory] [-e|--environment <env>] [-g|--groupid <id>] [-p|--payments <mode>] [-n|--name <place-name>]",
  async run(args) {
    const positional: string[] = [];
    const options: { environment?: string; groupId?: string; payments?: string; name?: string } = {};
    let all = false;

    for (let i = 0; i < args.length; i++) {
      const arg = args[i];
      const key = FLAGS[arg];

      if (key) {
        const value = args[++i];
        if (!value) {
          throw new Error(`"${arg}" for "deploy" requires a value`);
        }
        options[key] = value;
        continue;
      }

      if (ALL_FLAGS.includes(arg)) {
        all = true;
        continue;
      }

      if (arg.startsWith("-")) {
        throw new Error(`Unknown option "${arg}" for "deploy"`);
      }

      positional.push(arg);
    }

    const deployOptions = {
      environment: options.environment || "dev",
      groupId: options.groupId,
      payments: options.payments,
      name: options.name,
    };

    if (all) {
      const target = positional[0] || ".";
      const names = ListProjects(target);
      if (names.length === 0) {
        throw new Error(`No projects found to deploy.`);
      }

      const failures: string[] = [];
      for (const name of names) {
        try {
          await DeployProject(name, target, deployOptions);
        } catch (err) {
          failures.push(name);
          log.error(`Failed to deploy "${name}": ${err instanceof Error ? err.message : String(err)}`);
        }
      }

      if (failures.length > 0) {
        throw new Error(`Failed to deploy: ${failures.join(", ")}`);
      }
      return;
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"deploy" requires a project name, e.g. "jafa deploy my-project --environment dev", or use -a/--all to deploy all projects`);
    }

    await DeployProject(name, target || ".", deployOptions);
  },
};
