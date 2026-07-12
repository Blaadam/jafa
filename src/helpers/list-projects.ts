import { resolve } from "node:path";
import { readdirSync } from "node:fs";
import process from "node:process";

export function ListProjects(target: string): string[] {
  const cwd = resolve(process.cwd(), target);
  const projectsDir = resolve(cwd, "projects");

  try {
    return readdirSync(projectsDir, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name);
  } catch {
    return [];
  }
}
