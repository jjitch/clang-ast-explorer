import { Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { tauriInvoke } from "./backend/api";
import type { AstEntityLite, SourceRange } from "./backend/interface";

export function AstNode({
  node,
  highlightSourceRange,
}: {
  node: AstEntityLite;
  highlightSourceRange: (range: SourceRange) => void;
}) {
  const [revealedChildren, setRevealedChildren] = useState<
    AstEntityLite[] | null
  >(null);
  const [sourceRange, setSourceRange] = useState<SourceRange | null>(null);
  useEffect(() => {
    tauriInvoke("reveal_entity", {
      entityId: node.id,
    }).then((result) => {
      setSourceRange(result.source_range);
      setRevealedChildren(result.children);
    });
  }, [node.id]);
  const itemType =
    revealedChildren === null || revealedChildren.length === 0
      ? "leaf"
      : "branch";

  return (
    <TreeItem
      itemType={itemType}
      value={node.id}
      onClick={() => {
        if (sourceRange) {
          highlightSourceRange(sourceRange);
        }
      }}
    >
      <TreeItemLayout>
        {node.display_name} ({node.kind})
      </TreeItemLayout>
      {revealedChildren !== null && (
        <Tree>
          {revealedChildren.map((child) => (
            <AstNode
              key={child.id}
              node={child}
              highlightSourceRange={highlightSourceRange}
            />
          ))}
        </Tree>
      )}
    </TreeItem>
  );
}
