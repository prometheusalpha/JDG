import { ChevronDown, ChevronRight } from "lucide-react";
import { useState } from "react";
import { Checkbox } from "./ui/checkbox";

export interface FileNode {
  name: string;
  path: string;
  type: "file" | "folder";
  children?: FileNode[];
  selected?: boolean;
}

export interface FileTreeProps {
  node: FileNode;
  level: number;
  onToggle?: (path: string) => void;
  onSelect: (path: string, checked: boolean) => void;
}

export function FileTreeNode({
  node,
  level,
  onToggle,
  onSelect,
}: FileTreeProps) {
  const [expanded, setExpanded] = useState(true);

  const handleToggle = () => {
    if (node.type === "folder") {
      setExpanded(!expanded);
      onToggle?.(node.path);
    } else {
      onSelect(node.path, !node.selected);
    }
  };

  // Function to get the compressed name for packages with single child
  const getCompressedName = (
    currentNode: FileNode
  ): { name: string; node: FileNode } => {
    if (currentNode.type === "folder" && currentNode.children?.length === 1) {
      const child = currentNode.children[0];
      if (child.type === "folder") {
        const result = getCompressedName(child);
        return {
          name: `${currentNode.name}.${result.name}`,
          node: result.node,
        };
      }
    }
    return { name: currentNode.name, node: currentNode };
  };

  // Get compressed package name if applicable
  const { name: displayName, node: displayNode } = getCompressedName(node);

  return (
    <div className="select-none">
      <div
        className="flex items-center gap-2 py-1 hover:bg-accent/50 rounded-lg cursor-pointer"
        style={{ paddingLeft: `${level * 7 + 4}px` }}
        onClick={handleToggle}
      >
        {displayNode.type === "folder" && (
          <button className="h-4 w-4">
            {expanded ? (
              <ChevronDown className="h-4 w-4" />
            ) : (
              <ChevronRight className="h-4 w-4" />
            )}
          </button>
        )}
        {displayNode.type === "file" && (
          <Checkbox
            id={displayNode.path}
            checked={displayNode.selected}
            onCheckedChange={(checked) =>
              onSelect(displayNode.path, checked as boolean)
            }
            onClick={(e) => e.stopPropagation()}
          />
        )}
        <span className="text-sm">{displayName}</span>
      </div>
      {displayNode.type === "folder" && expanded && displayNode.children && (
        <div className="">
          {displayNode.children.map((child) => (
            <FileTreeNode
              key={child.path}
              node={child}
              level={level + 1}
              onToggle={onToggle}
              onSelect={onSelect}
            />
          ))}
        </div>
      )}
    </div>
  );
}
