import Fs from "node:fs";

export function ReadJsonFile<T>(filePath: string): T {
  const jsonString = Fs.readFileSync(filePath, "utf8");
  return JSON.parse(jsonString) as T;
}