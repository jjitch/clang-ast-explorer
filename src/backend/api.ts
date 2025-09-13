import { invoke, isTauri } from "@tauri-apps/api/core";
import type {
  EventCallback,
  Event as TauriEvent,
  UnlistenFn,
} from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { EventPayload, TauriCommands } from "./interface";
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

export function tauriListen<T extends keyof EventPayload>(
  event: T,
  callback: EventCallback<TauriEvent<EventPayload[T]>>,
): Promise<UnlistenFn> {
  if (isTauri()) {
    const appWebview = getCurrentWebviewWindow();
    return appWebview.listen<TauriEvent<EventPayload[T]>>(event, callback);
  } else {
    const mockCallback = (e: Event) => {
      callback((e as CustomEvent).detail);
    };
    document.addEventListener(event, mockCallback);
    return Promise.resolve(() => {
      document.removeEventListener(event, mockCallback);
    });
  }
}
