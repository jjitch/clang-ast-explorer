import { Tree } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { AstNode } from "./AstNode";
import { tauriListen } from "./backend/api";
import type { AstEntityLite } from "./backend/interface";

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

export function AstExplorer() {
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
    <div style={{ height: "95%", display: "flex", flexDirection: "column" }}>
      {astState.id === "not-parsed" && <div>AST not started</div>}
      {astState.id === "parsing" && <div>Parsing AST...</div>}
      {astState.id === "ready" && (
        <Tree size="small" style={{ overflowY: "scroll", flexGrow: 1 }}>
          <AstNode key={astState.entity.id} node={astState.entity} />
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
