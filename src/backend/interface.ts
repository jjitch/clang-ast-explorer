export type ParseSourceArg = {
  sourceCode: string;
};

export type RevealEntityArg = {
  entityId: string;
};

export type SourceRange = {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
};

export type AstEntityFull = {
  properties: Array<[string, string]>;
  children: AstEntityLite[];
  source_range: SourceRange | null;
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
