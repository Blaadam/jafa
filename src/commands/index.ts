import type { CommandDefinition } from "./types.js";
import { helloCommand } from "./hello.js";
import { initCommand } from "./init.js";
import { newCommand } from "./new.js";

export type { CommandDefinition } from "./types.js";

export const commands: CommandDefinition[] = [helloCommand, initCommand, newCommand];

export function findCommand(name: string): CommandDefinition | undefined {
  return commands.find((c) => c.name === name || c.aliases?.includes(name));
}
