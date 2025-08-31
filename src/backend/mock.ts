import type { FileInfo, TauriCommands } from "./interface";

type TauriIF = {
  [K in keyof TauriCommands]: (
    args: TauriCommands[K]["args"],
  ) => Promise<TauriCommands[K]["return"]>;
};

export const mock: TauriIF = {
  submit_file: (_args: FileInfo) => {
    return new Promise((resolve) => {
      resolve("This is mock.");
    });
  },
};
