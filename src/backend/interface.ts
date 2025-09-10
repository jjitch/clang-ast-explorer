export type ParseSourceArg = {
  sourceCode: string;
};

export type ParseSourceReturn = {
  diagnostics: string;
};

export type TauriCommands = {
  parse_source: {
    args: ParseSourceArg;
    return: ParseSourceReturn;
  };
};
