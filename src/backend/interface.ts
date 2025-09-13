export type ParseSourceArg = {
  sourceCode: string;
};

export type TauriCommands = {
  parse_source: {
    args: ParseSourceArg;
    return: null;
  };
};

export type EventPayload = {
  "ast-ready": string;
};
