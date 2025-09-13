import react from "react";
import { AstExplorer } from "./AstExplorer";
import { tauriInvoke } from "./backend/api";
import { Editor, type EditorHandle } from "./Editor";

function App() {
  const [invokeResponse, setInvokeResponse] = react.useState("");
  const editorRef = react.useRef<EditorHandle>(null);

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
            tauriInvoke("parse_source", { sourceCode: value })
              .then((response) => {
                setInvokeResponse(response);
              })
              .catch((error) => {
                setInvokeResponse(`Error: ${error}`);
              });
          }
        }}
      >
        Process File
      </button>
      <div>Invoke response: {invokeResponse}</div>
      <div style={{ display: "flex", gap: "20px", height: "100%" }}>
        <Editor ref={editorRef} />
        <AstExplorer />
      </div>
    </div>
  );
}

export default App;
