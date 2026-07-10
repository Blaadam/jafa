import type { CommandDefinition } from "./types.js";

export const helloCommand: CommandDefinition = {
  name: "hello",
  aliases: ["hi", "hey"],
  description: "Print a friendly greeting",
  run(args) {
    const name = args.find((arg) => !arg.startsWith("-")) ?? "world";
    console.log(`Hello, ${name}!`);
  },
};
