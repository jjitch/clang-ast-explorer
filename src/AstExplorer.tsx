import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export function AstExplorer() {
  const appWebview = getCurrentWebviewWindow();
  appWebview.listen<string>("ast-ready", (event) => {
    console.log("AST ready event received in AstExplorer");
    console.log("Event:", event);
  });

  return (
    <div>
      <h1>AST Explorer</h1>
    </div>
  );
}
