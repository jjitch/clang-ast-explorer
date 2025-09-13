import { useEffect, useState } from "react";
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
    <div>
      <h1>AST Explorer</h1>
      {astState.id === "not-parsed" && <div>AST not started</div>}
      {astState.id === "parsing" && <div>Parsing AST...</div>}
      {astState.id === "ready" && (
        <div>
          <p>AST ready!</p>
          <p>Entity ID: {astState.entity.id}</p>
          <p>Entity Kind: {astState.entity.kind}</p>
        </div>
      )}
      {astState.id === "error" && <div>Error: {astState.message}</div>}
      {astState.id !== "not-parsed" &&
        astState.id !== "parsing" &&
        astState.id !== "ready" &&
        astState.id !== "error" && <div>Unknown state</div>}
    </div>
  );
}
