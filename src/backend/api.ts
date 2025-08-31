import { invoke, isTauri } from "@tauri-apps/api/core";
import type { TauriCommands } from "./interface";
import { mock } from "./mock";

export function tauriInvoke<T extends keyof TauriCommands>(
  command: T,
  args: TauriCommands[T]["args"],
): Promise<TauriCommands[T]["return"]> {
  if (isTauri()) {
    console.log("Invoking...");
    return invoke(command, args);
  } else {
    console.log("Mocking...");
    return mock[command](args);
  }
}
