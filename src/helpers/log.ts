import chalk from "chalk";

export const levels = {
  verbose: 0,
  trace: 1,
  info: 2,
  warn: 3,
  error: 4,
} as const;

const level: number = levels.info;

const makePrefix = (slot: string): string =>
  `${chalk.bold.cyan("jafa ")}${chalk.gray("[")}${slot}${chalk.gray("]")}`;

function write(threshold: number, label: string, message: unknown): void {
  if (level <= threshold) console.log(makePrefix(label), message);
}

const verbose = (message: unknown): void =>
  write(levels.verbose, chalk.magenta("verbose"), message);

const trace = (message: unknown): void =>
  write(levels.trace, chalk.blue("trace"), message);

const info = (message: unknown): void =>
  write(levels.info, chalk.greenBright("info"), message);

const warn = (message: unknown): void =>
  write(levels.warn, chalk.yellow("WARN"), message);

const error = (message: unknown): void =>
  write(levels.error, chalk.red("ERR!"), message);

export const log = {
  verbose,
  trace,
  info,
  warn,
  error,
};