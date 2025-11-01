import { Tree, TreeItem, TreeItemLayout } from "@fluentui/react-components";
import { useState } from "react";
import { tauriInvoke } from "./backend/api";
import type { AstEntityLite } from "./backend/interface";

export function AstNode({ node }: { node: AstEntityLite }) {
  const [revealedChildren, setRevealedChildren] = useState<
    AstEntityLite[] | null
  >(null);
  const [open, setOpen] = useState(false);
  const itemType = revealedChildren === null ? "leaf" : "branch";

  return (
    <TreeItem
      itemType={itemType}
      value={node.id}
      onClick={async () => {
        if (revealedChildren === null) {
          const result = await tauriInvoke("reveal_entity", {
            entityId: node.id,
          });
          setRevealedChildren(result.children);
        }
        setOpen(!open);
      }}
    >
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
