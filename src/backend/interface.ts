export type ParseSourceArg = {
  sourceCode: string;
};

export type TauriCommands = {
  parse_source: {
    args: ParseSourceArg;
    return: null;
  };
};

export type AstEntityLite = {
  id: string;
  kind: string;
};

export type EventPayload = {
  "ast-ready": AstEntityLite;
};
