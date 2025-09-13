export type ParseSourceArg = {
  sourceCode: string;
};

export type RevealEntityArg = {
  entityId: string;
};

export type AstEntityFull = {
  id: string;
  kind: string;
  children: AstEntityLite[];
};

export type TauriCommands = {
  parse_source: {
    args: ParseSourceArg;
    return: null;
  };
  reveal_entity: {
    args: RevealEntityArg;
    return: AstEntityFull;
  };
};

export type AstEntityLite = {
  id: string;
  kind: string;
};

export type EventPayload = {
  "ast-ready": AstEntityLite;
};
