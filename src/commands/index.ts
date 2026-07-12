import type { CommandDefinition } from "./types.js";
import { initCommand } from "./init.js";
import { newCommand } from "./new.js";
import { buildCommand } from "./build.js";
import { openCommand } from "./open.js";
import { serveCommand } from "./serve.js";
import { devCommand } from "./dev.js";
import { deployCommand } from "./deploy.js";
import { sourcemapCommand } from "./sourcemap.js";

export type { CommandDefinition } from "./types.js";

export const commands: CommandDefinition[] = [initCommand, newCommand, buildCommand, openCommand, serveCommand, devCommand, deployCommand, sourcemapCommand];

export function findCommand(name: string): CommandDefinition | undefined {
  return commands.find((c) => c.name === name || c.aliases?.includes(name));
}
