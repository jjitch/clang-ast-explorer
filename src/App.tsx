import react, { useEffect } from "react";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import { AstExplorer } from "./AstExplorer";
import { tauriInvoke, tauriListen } from "./backend/api";
import type { SourceCode } from "./backend/interface";
import { TokenDecoEditor, type TokenDecoEditorRef } from "./TokenDecoEditor";

// import { Editor, type EditorHandle } from "./Editor";

function App() {
  const editorRef = react.useRef<TokenDecoEditorRef | null>(null);
  const [sourceCode, setSourceCode] = react.useState<SourceCode>({
    path: "inmemory://main.cpp",
    content: "",
  });
  useEffect(() => {
    const unmount = tauriListen("file-picked", (event) => {
      setSourceCode(event.payload);
    });
    return () => {
      unmount.then((fn) => fn());
    };
  }, []);

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
          const value = editorRef.current?.getValue();
          if (value) {
            tauriInvoke("parse_source", { sourceCode: value });
          }
        }}
      >
        Process File
      </button>
      <PanelGroup direction="horizontal" autoSaveId="main-panel-group">
        <Panel>
          <TokenDecoEditor
            ref={editorRef}
            filePath={sourceCode.path}
            fileContent={sourceCode.content}
          />
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
