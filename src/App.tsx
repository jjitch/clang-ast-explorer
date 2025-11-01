import react from "react";
import { AstExplorer } from "./AstExplorer";
import { tauriInvoke } from "./backend/api";
import { TokenDecoEditor, type TokenDecoEditorRef } from "./TokenDecoEditor";

// import { Editor, type EditorHandle } from "./Editor";

function App() {
  const editorRef = react.useRef<TokenDecoEditorRef | null>(null);

  return (
    <div
      style={{
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        gap: "10px",
        padding: "10px",
      }}
    >
      <button
        type="button"
        onClick={() => {
          const value = editorRef.current?.();
          if (value) {
            tauriInvoke("parse_source", { sourceCode: value });
          }
        }}
      >
        Process File
      </button>
      <div style={{ display: "flex", gap: "20px", height: "100%" }}>
        <TokenDecoEditor ref={editorRef} filePath="inmemory://main.cpp" />
        <AstExplorer />
      </div>
    </div>
  );
}

export default App;
