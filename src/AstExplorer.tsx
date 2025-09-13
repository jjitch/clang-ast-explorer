import { useEffect, useState } from "react";
import { tauriListen } from "./backend/api";

type AstParsing = {
  id: "parsing";
};

type AstNotParsed = {
  id: "not-parsed";
};

type AstReady = {
  id: "ready";
  rootEntityId: string;
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
      setAstState({ id: "ready", rootEntityId: event.payload });
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
        <div>AST ready! Root entity ID: {astState.rootEntityId}</div>
      )}
      {astState.id === "error" && <div>Error: {astState.message}</div>}
      {astState.id !== "not-parsed" &&
        astState.id !== "parsing" &&
        astState.id !== "ready" &&
        astState.id !== "error" && <div>Unknown state</div>}
    </div>
  );
}
