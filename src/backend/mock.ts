import type { EventPayload, TauriCommands } from "./interface";

type TauriIF = {
  [K in keyof TauriCommands]: (
    args: TauriCommands[K]["args"],
  ) => Promise<TauriCommands[K]["return"]>;
};

export const mock: TauriIF = {
  parse_source: (_args: { sourceCode: string }) => {
    return new Promise((resolve) => {
      console.log("Mocking parse_source...");
      setTimeout(() => {
        console.log("1 second passed, dispatching ast-ready event...");
        emit("ast-ready", "This is mock AST data.");
        resolve("This is mock parse source resolve.");
      }, 1000);
    });
  },
};

function emit<K extends keyof EventPayload>(
  event: K,
  payload: EventPayload[K],
) {
  document.dispatchEvent(
    new CustomEvent<{ payload: EventPayload[K] }>(event, {
      detail: { payload },
    }),
  );
}
