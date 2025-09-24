import { Editor } from "@monaco-editor/react";
import type * as monaco from "monaco-editor";
import { forwardRef, useImperativeHandle, useRef } from "react";

type TokenDecoEditorProps = {
  filePath: "inmemory://main.cpp";
};

export type TokenDecoEditorRef = () => string;

export const TokenDecoEditor = forwardRef<
  TokenDecoEditorRef,
  TokenDecoEditorProps
>(({ filePath }, ref) => {
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
      model = monacoInstance.editor.createModel("", "cpp", uri);
    }
  };
  return (
    <Editor language="cpp" theme="vs-dark" onMount={onMount} width={"50vw"} />
  );
});
