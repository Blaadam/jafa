import process from "node:process";
import shell from "shelljs";

export default function exec(
  command: string,
  silent = true,
  cwd: string = process.cwd(),
): Promise<string> {
  return new Promise((resolve, reject) => {
    const out = shell.exec(command, { silent, cwd });

    if (out.stderr) reject(new Error(out.stderr.trim()));
    else resolve(out.stdout.trim());
  });
}
