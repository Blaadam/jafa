#!/usr/bin/env node

import process from "node:process";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

type Command = "help" | "version" | "init";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// dist/cli.js -> ../package.json
const pkgPath = resolve(__dirname, "../package.json");
const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as {
  name?: string;
  version?: string;
};

const CLI_NAME = pkg.name ?? "jafa";
const CLI_VERSION = pkg.version ?? "0.0.0";

function printHelp(): void {
  console.log(`
${CLI_NAME} - A CLI tool for Roblox repository management

Usage:
  jafa <command> [options]

Commands:
  init                Create a new jafa project in the current directory
  help                Show help information
  version             Show CLI version

Options:
  -h, --help          Show help
  -v, --version       Show version

Examples:
  jafa init
  jafa --help
  jafa --version
`.trim());
}

function printVersion(): void {
  console.log(`${CLI_NAME} v${CLI_VERSION}`);
}

function isFlag(value: string): boolean {
  return value.startsWith("-");
}

function fail(message: string): never {
  console.error(`Error: ${message}`);
  console.error(`Run "jafa --help" for usage.`);
  process.exit(1);
}

function parseCommand(args: string[]): { command: Command; rest: string[] } {
  if (args.length === 0) return { command: "help", rest: [] };

  const [first, ...rest] = args;

  if (first === "-h" || first === "--help") return { command: "help", rest };
  if (first === "-v" || first === "--version") return { command: "version", rest };

  if (first === "help") return { command: "help", rest };
  if (first === "version") return { command: "version", rest };
  if (first === "init") return { command: "init", rest };

  if (isFlag(first)) fail(`Unknown option "${first}"`);
  fail(`Unknown command "${first}"`);
}

function runInit(args: string[]): void {
  // Simple strict handling for now; expand later
  const unknownFlags = args.filter(isFlag);
  if (unknownFlags.length > 0) {
    fail(`Unknown option(s) for "init": ${unknownFlags.join(", ")}`);
  }

  console.log("Initializing jafa in this project...");
  // TODO: add your real init logic here
  console.log("Done.");
}

function main(): void {
  const userArgs = process.argv.slice(2);
  const { command, rest } = parseCommand(userArgs);

  switch (command) {
    case "help":
      printHelp();
      return;
    case "version":
      printVersion();
      return;
    case "init":
      runInit(rest);
      return;
    default: {
      const _exhaustive: never = command;
      throw new Error(`Unhandled command: ${_exhaustive}`);
    }
  }
}

main();