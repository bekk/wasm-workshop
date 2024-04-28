import { consoleLogInJs, logFunctionInJs } from "./log";

export function run(fromJs: string): string {
  consoleLogInJs("console.log: Hello from console.log.");
  logFunctionInJs("functionInJs: Hello from global function.");
  return `${fromJs} is done.`;
}
