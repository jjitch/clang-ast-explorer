export type FileInfo = {
  sourceText: string;
  fileName: string;
};

export type TauriCommands = {
  submit_file: {
    args: FileInfo;
    return: string;
  };
};
