import { consoleLogInJs, logFunctionInJs } from "./log";

export function run(fromJs: string): string {
  logFunctionInJs("functionInJs: Hello from global function.");
  consoleLogInJs("console.log: Hello from console.log.");
  return `${fromJs} is done.`;
}
