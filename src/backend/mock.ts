import { SHA1 } from "crypto-es";
import type { EventPayload, RevealEntityArg, TauriCommands } from "./interface";

type TauriIF = {
  [K in keyof TauriCommands]: (
    args: TauriCommands[K]["args"],
  ) => Promise<TauriCommands[K]["return"]>;
};

export const mock: TauriIF = {
  parse_source: (args: { sourceCode: string }) => {
    return new Promise((resolve) => {
      console.log("Mocking parse_source...");
      const hash = SHA1(args.sourceCode).toString();
      setTimeout(() => {
        console.log("1 second passed, dispatching ast-ready event...");
        emit("ast-ready", {
          id: hash,
          kind: "mock-kind",
          display_name: "Mock Display Name",
        });
        resolve(null);
      }, 10);
    });
  },
  reveal_entity: (args: RevealEntityArg) => {
    return new Promise((resolve) => {
      console.log("Mocking reveal_entity...");
      const id = args.entityId;
      const children = [];
      for (let i = 0; i < 100; i++) {
        children.push({
          id: `${id}/${i}`,
          kind: "mock-kind",
          display_name: `Mock Child ${i}`,
        });
      }
      resolve({ children });
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
