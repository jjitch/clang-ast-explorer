import { Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { tauriInvoke } from "./backend/api";
import type { AstEntityLite } from "./backend/interface";

export function AstNode({ node }: { node: AstEntityLite }) {
  const [revealedChildren, setRevealedChildren] = useState<
    AstEntityLite[] | null
  >(null);
  useEffect(() => {
    tauriInvoke("reveal_entity", {
      entityId: node.id,
    }).then((result) => {
      setRevealedChildren(result.children);
    });
  }, [node.id]);
  const itemType =
    revealedChildren === null || revealedChildren.length === 0
      ? "leaf"
      : "branch";

  return (
    <TreeItem itemType={itemType} value={node.id}>
      <TreeItemLayout>
        {node.display_name} ({node.kind})
      </TreeItemLayout>
      {revealedChildren !== null && (
        <Tree>
          {revealedChildren.map((child) => (
            <AstNode key={child.id} node={child} />
          ))}
        </Tree>
      )}
    </TreeItem>
  );
}
