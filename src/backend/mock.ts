import type { TauriCommands } from "./interface";

type TauriIF = {
  [K in keyof TauriCommands]: (
    args: TauriCommands[K]["args"],
  ) => Promise<TauriCommands[K]["return"]>;
};

export const mock: TauriIF = {
  parse_source: (_args: { sourceCode: string }) => {
    return new Promise((resolve) => {
      resolve({
        diagnostics: "This is mock.",
      });
    });
  },
};
