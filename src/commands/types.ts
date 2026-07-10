export interface CommandDefinition {
  name: string;
  aliases?: string[];
  description: string;
  run(args: string[]): void | Promise<void>;
}
