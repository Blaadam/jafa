import type { CommandDefinition } from "./types.js";

export const initCommand: CommandDefinition = {
  name: "init",
  description: "Create a new jafa project in the current directory",
  run(args) {
    const unknownFlags = args.filter((arg) => arg.startsWith("-"));
    if (unknownFlags.length > 0) {
      throw new Error(`Unknown option(s) for "init": ${unknownFlags.join(", ")}`);
    }

    console.log("Initializing jafa in this project...");
    // TODO: add your real init logic here
    console.log("Done.");
  },
};
