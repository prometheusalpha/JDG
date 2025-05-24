import { FileNode } from "@/components/file-tree";

export const search = (files: FileNode[], searchTerm: string): FileNode[] => {
  const results: FileNode[] = [];
  if (!searchTerm) {
    return files;
  }
  for (const file of files) {
    if (!file.children || file.children.length === 0) {
      if (file.name.toLowerCase().includes(searchTerm.toLowerCase())) {
        results.push(file);
      }
    } else {
      const childResults = search(file.children, searchTerm);
      results.push(...childResults);
    }
  }
  return results;
};
