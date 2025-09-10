import react from "react";
import { tauriInvoke } from "./backend/api";
import { Editor, type EditorHandle } from "./Editor";

function App() {
  const [invokeResponse, setInvokeResponse] = react.useState("");
  const editorRef = react.useRef<EditorHandle>(null);

  return (
    <div>
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
      <p>{invokeResponse}</p>
      <Editor ref={editorRef} />
    </div>
  );
}

export default App;
