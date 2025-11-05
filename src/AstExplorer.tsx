import { Tree } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { AstNode } from "./AstNode";
import { tauriListen } from "./backend/api";
import type { AstEntityLite, SourceRange } from "./backend/interface";

type AstParsing = {
  id: "parsing";
};

type AstNotParsed = {
  id: "not-parsed";
};

type AstReady = {
  id: "ready";
  entity: AstEntityLite;
};

type AstError = {
  id: "error";
  message: string;
};

type AstState = AstNotParsed | AstParsing | AstReady | AstError;

type AstExplorerProps = {
  highlightSourceRange: (range: SourceRange) => void;
};

export function AstExplorer({ highlightSourceRange }: AstExplorerProps) {
  const [astState, setAstState] = useState<AstState>({ id: "not-parsed" });
  useEffect(() => {
    const unmount = tauriListen("ast-ready", (event) => {
      console.log("AST ready event received:", event);
      setAstState({ id: "ready", entity: event.payload });
    });
    return () => {
      unmount.then((fn) => fn());
    };
  }, []);
  return (
    // "height: 95%" prevents the tree from being cut off.
    // This is a workaround; ideal solution is being investigated.
    <div style={{ height: "95%", display: "flex", flexDirection: "column" }}>
      {astState.id === "not-parsed" && <div>AST not started</div>}
      {astState.id === "parsing" && <div>Parsing AST...</div>}
      {astState.id === "ready" && (
        <Tree
          size="small"
          style={{ overflowY: "scroll", flexGrow: 1 }}
          aria-label="abst-syntax-tree"
        >
          <AstNode
            key={astState.entity.id}
            node={astState.entity}
            highlightSourceRange={highlightSourceRange}
          />
        </Tree>
      )}
      {astState.id === "error" && <div>Error: {astState.message}</div>}
      {astState.id !== "not-parsed" &&
        astState.id !== "parsing" &&
        astState.id !== "ready" &&
        astState.id !== "error" && <div>Unknown state</div>}
    </div>
  );
}
