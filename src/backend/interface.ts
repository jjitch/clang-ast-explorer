export type ParseSourceArg = {
  sourceCode: string;
};

export type TauriCommands = {
  parse_source: {
    args: ParseSourceArg;
    return: string;
  };
};
