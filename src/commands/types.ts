export interface CommandDefinition {
  name: string;
  aliases?: string[];
  usage?: string;
  description: string;
  run(args: string[]): void | Promise<void>;
}
