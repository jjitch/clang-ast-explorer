import { useEffect } from "react";
import { tauriListen } from "./backend/api";

export function AstExplorer() {
  useEffect(() => {
    const unmount = tauriListen("ast-ready", (event) => {
      console.log("AST ready event received:", event);
    });
    return () => {
      unmount.then((fn) => fn());
    };
  }, []);
  return (
    <div>
      <h1>AST Explorer</h1>
    </div>
  );
}
