import { Editor } from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import { forwardRef, useImperativeHandle, useRef } from "react";

import "./TokenDecoEditor.css";

type TokenDecoEditorProps = {
  filePath: string | "inmemory://main.cpp";
  fileContent: string;
};

export type TokenDecoEditorRef = {
  getValue: () => string;
  setHighlightedRange: (range: monaco.IRange) => void;
};

export const TokenDecoEditor = forwardRef<
  TokenDecoEditorRef,
  TokenDecoEditorProps
>(({ filePath, fileContent }, ref) => {
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const monacoRef = useRef<typeof monaco | null>(null);
  const decoIdsRef = useRef<string[]>([]);

  const setHighlightedRange = (range: monaco.IRange) => {
    const editor = editorRef.current;
    const monacoInstance = monacoRef.current;
    if (!editor || !monacoInstance) return;
    const uri = monacoInstance.Uri.parse(filePath);
    const model = monacoInstance.editor.getModel(uri);
    if (!model) return;

    decoIdsRef.current = model.deltaDecorations(decoIdsRef.current, [
      {
        range,
        options: { inlineClassName: "myLineDecoration" },
      },
    ]);
  };

  useImperativeHandle(ref, () => ({
    getValue: () => {
      const editor = editorRef.current;
      return editor ? editor.getValue() : "";
    },
    setHighlightedRange,
  }));
  const onMount = (
    editor: monaco.editor.IStandaloneCodeEditor,
    monacoInstance: typeof monaco,
  ) => {
    editorRef.current = editor;
    monacoRef.current = monacoInstance;
    const uri = monacoInstance.Uri.parse(filePath);
    let model = monacoInstance.editor.getModel(uri);
    if (!model) {
      model = monacoInstance.editor.createModel(fileContent, "cpp", uri);
      editor.setModel(model);
    }
  };

  return (
    <Editor
      language="cpp"
      path={filePath}
      theme="vs-dark"
      onMount={onMount}
      value={fileContent}
    />
  );
});
