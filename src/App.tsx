import react from "react";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
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
      <PanelGroup direction="horizontal" autoSaveId="main-panel-group">
        <Panel>
          <TokenDecoEditor ref={editorRef} filePath="inmemory://main.cpp" />
        </Panel>
        <PanelResizeHandle style={{ width: "5px" }} />
        <Panel>
          <AstExplorer />
        </Panel>
      </PanelGroup>
    </div>
  );
}

export default App;
