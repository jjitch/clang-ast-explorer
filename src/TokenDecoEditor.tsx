import { Editor } from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import { forwardRef, useImperativeHandle, useRef } from "react";

type TokenDecoEditorProps = {
  filePath: string | "inmemory://main.cpp";
  fileContent?: string;
};

export type TokenDecoEditorRef = () => string;

export const TokenDecoEditor = forwardRef<
  TokenDecoEditorRef,
  TokenDecoEditorProps
>(({ filePath, fileContent }, ref) => {
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const monacoRef = useRef<typeof monaco | null>(null);

  useImperativeHandle(ref, () => {
    return () => editorRef.current?.getValue() || "";
  });
  const onMount = (
    editor: monaco.editor.IStandaloneCodeEditor,
    monacoInstance: typeof monaco,
  ) => {
    editorRef.current = editor;
    monacoRef.current = monacoInstance;
    const uri = monacoInstance.Uri.parse(filePath);
    let model = monacoInstance.editor.getModel(uri);
    if (!model) {
      model = monacoInstance.editor.createModel(fileContent || "", "cpp", uri);
    }
  };

  return (
    <Editor
      language="cpp"
      theme="vs-dark"
      onMount={onMount}
      value={fileContent}
    />
  );
});
