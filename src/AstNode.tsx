import { useState } from "react";
import { tauriInvoke } from "./backend/api";
import type { AstEntityLite } from "./backend/interface";

export function AstNode({ node }: { node: AstEntityLite }) {
  const [revealedChildren, setRevealedChildren] = useState<
    AstEntityLite[] | null
  >(null);
  const [open, setOpen] = useState(false);

  return (
    <li>
      <button
        type="button"
        onClick={() => {
          if (revealedChildren === null) {
            tauriInvoke("reveal_entity", { entityId: node.id }).then((res) => {
              setRevealedChildren(res.children || []);
            });
          }
          setOpen(!open);
        }}
      >
        {`${open ? "▼" : "►"} [${node.kind}] ${node.display_name}`}
      </button>
      {revealedChildren && revealedChildren.length > 0 && open && (
        <ul>
          {revealedChildren.map((child) => (
            <AstNode key={child.id} node={child} />
          ))}
        </ul>
      )}
    </li>
  );
}
