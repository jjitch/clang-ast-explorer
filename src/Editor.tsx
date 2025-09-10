import * as monaco from "monaco-editor/esm/vs/editor/editor.api";
import { forwardRef, useEffect, useImperativeHandle, useRef } from "react";

export type EditorHandle = {
  getValue: () => string | undefined;
};

export const Editor = forwardRef<EditorHandle>((_, ref) => {
  const monacoEl = useRef<HTMLDivElement | null>(null);
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);

  useImperativeHandle(
    ref,
    () => ({
      getValue: () => editorRef.current?.getValue(),
    }),
    [],
  );

  useEffect(() => {
    if (monacoEl.current && !editorRef.current) {
      editorRef.current = monaco.editor.create(monacoEl.current, {
        value: ["int main() {", "  return 0;", "}"].join("\n"),
        language: "cpp",
        automaticLayout: true,
        theme: "vs-dark",
      });
    }
    return () => {
      editorRef.current?.dispose();
      editorRef.current = null;
    };
  }, []);

  return <div ref={monacoEl} style={{ height: "50vh" }} />;
});
