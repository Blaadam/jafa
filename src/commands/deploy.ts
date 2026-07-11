import type { CommandDefinition } from "./types.js";
import { DeployProject } from "../functions/deploy-project.js";

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

export const deployCommand: CommandDefinition = {
  name: "deploy",
  description: "Build and deploy a project to Roblox using Mantle",
  usage: "deploy <project-name> [target-directory] [-e|--environment <env>] [-g|--groupid <id>] [-p|--payments <mode>] [-n|--name <place-name>]",
  run(args) {
    const positional: string[] = [];
    const options: { environment?: string; groupId?: string; payments?: string; name?: string } = {};

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

      if (arg.startsWith("-")) {
        throw new Error(`Unknown option "${arg}" for "deploy"`);
      }

      positional.push(arg);
    }

    const [name, target] = positional;
    if (!name) {
      throw new Error(`"deploy" requires a project name, e.g. "jafa deploy my-project --environment dev"`);
    }

    DeployProject(name, target || ".", {
      environment: options.environment || "dev",
      groupId: options.groupId,
      payments: options.payments,
      name: options.name,
    });
  },
};
