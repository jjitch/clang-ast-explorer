import { useState } from "react";
import { tauriInvoke } from "./backend/api";
import { AstEntityLite } from "./backend/interface";

export function AstNode({ node }: { node: AstEntityLite }) {
  const [revealedChildren, setRevealedChildren] = useState<
    AstEntityLite[] | null
  >(null);

  return (
    <ul>
      <li>
        <strong>{node.kind}</strong>: {node.id}
        <button
          type="button"
          onClick={() => {
            if (revealedChildren === null) {
              tauriInvoke("reveal_entity", { entityId: node.id }).then(
                (res) => {
                  setRevealedChildren(res.children || []);
                },
              );
            } else {
              setRevealedChildren(null);
            }
          }}
        >
          {revealedChildren === null ? "Reveal Children" : "Hide Children"}
        </button>
        {revealedChildren && revealedChildren.length > 0 && (
          <ul>
            {revealedChildren.map((child) => (
              <AstNode key={child.id} node={child} />
            ))}
          </ul>
        )}
      </li>
    </ul>
  );
}
