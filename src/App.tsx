import react from "react";
import { tauriInvoke } from "./backend/api";
import type { ParseSourceArg } from "./backend/interface";

function App() {
  const [fileInfo, setFileInfo] = react.useState<ParseSourceArg | null>(null);
  const [invokeResponse, setInvokeResponse] = react.useState("");

  return (
    <div>
      <input
        type="file"
        onChange={(e) => {
          if (e.target.files && e.target.files.length > 0) {
            e.target.files[0].text().then((text) => {
              setFileInfo({
                sourceCode: text,
              });
            });
          }
        }}
      />
      <button
        type="button"
        onClick={() => {
          if (fileInfo) {
            tauriInvoke("parse_source", fileInfo)
              .then((response) => {
                setInvokeResponse(String(response));
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
      <pre>{fileInfo?.sourceCode}</pre>
    </div>
  );
}

export default App;
