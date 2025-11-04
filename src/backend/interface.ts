export type ParseSourceArg = {
  sourceCode: string;
};

export type RevealEntityArg = {
  entityId: string;
};

export type AstEntityFull = {
  properties: Array<[string, string]>;
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
  display_name: string;
};

export type SourceCode = {
  path: string;
  content: string;
};

export type EventPayload = {
  "ast-ready": AstEntityLite;
  "file-picked": SourceCode;
};
