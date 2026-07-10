import type { CommandDefinition } from "./types.js";
import { initCommand } from "./init.js";
import { newCommand } from "./new.js";
import { buildCommand } from "./build.js";

export type { CommandDefinition } from "./types.js";

export const commands: CommandDefinition[] = [initCommand, newCommand, buildCommand];

export function findCommand(name: string): CommandDefinition | undefined {
  return commands.find((c) => c.name === name || c.aliases?.includes(name));
}
