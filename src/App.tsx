import react from "react";
import { tauriInvoke } from "./backend/api";
import type { FileInfo } from "./backend/interface";

function App() {
  const [fileInfo, setFileInfo] = react.useState<FileInfo | null>(null);
  const [invokeResponse, setInvokeResponse] = react.useState("");

  return (
    <div>
      <input
        type="file"
        onChange={(e) => {
          if (e.target.files && e.target.files.length > 0) {
            const fileName = e.target.files[0].name;
            e.target.files[0].text().then((text) => {
              setFileInfo({
                sourceText: text,
                fileName: fileName,
              });
            });
          }
        }}
      />
      <button
        type="button"
        onClick={() => {
          if (fileInfo) {
            tauriInvoke("submit_file", fileInfo)
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
      <pre>{fileInfo?.sourceText}</pre>
    </div>
  );
}

export default App;
