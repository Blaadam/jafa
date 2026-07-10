#!/usr/bin/env node

import process from "node:process";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { commands, findCommand } from "./commands/index.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const pkgPath = resolve(__dirname, "../package.json");
const pkg = JSON.parse(readFileSync(pkgPath, "utf8")) as {
  name?: string;
  version?: string;
};

const CLI_NAME = pkg.name ?? "jafa";
const CLI_VERSION = pkg.version ?? "0.0.0";

function printHelp(): void {
  const commandLines = commands
    .map((c) => {
      const aliases = c.aliases?.length ? ` (aliases: ${c.aliases.join(", ")})` : "";
      return `  ${c.name.padEnd(20)}${c.description}${aliases}`;
    })
    .join("\n");

  const examples = commands
    .filter((c) => c.usage)
    .map((c) => `  jafa ${c.usage}`)
    .join("\n");

  console.log(`
${CLI_NAME} - A CLI tool for Roblox repository management

Usage:
  jafa <command> [options]

Commands:
  help                Show help information
  version             Show CLI version
${commandLines}

Options:
  -h, --help          Show help
  -v, --version       Show version

Examples:
${examples}
  jafa --help
  jafa --version
`.trim());
}

function printVersion(): void {
  console.log(`${CLI_NAME} v${CLI_VERSION}`);
}

function fail(message: string): never {
  console.error(`Error: ${message}`);
  console.error(`Run "jafa --help" for usage.`);
  process.exit(1);
}

async function main(): Promise<void> {
  const [first, ...rest] = process.argv.slice(2);

  if (!first || first === "-h" || first === "--help" || first === "help") {
    printHelp();
    return;
  }
  if (first === "-v" || first === "--version" || first === "version") {
    printVersion();
    return;
  }

  const command = findCommand(first);
  if (!command) {
    fail(`Unknown command "${first}"`);
  }

  try {
    await command.run(rest);
  } catch (err) {
    fail(err instanceof Error ? err.message : String(err));
  }
}

main();
