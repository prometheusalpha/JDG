import { FileNode } from "@/components/file-tree";
import { create } from "zustand";

interface FileStore {
  files: FileNode[];
  setFiles: (files: FileNode[]) => void;
  selectedFiles: string[];
  reset: () => void;
}

const getSelected = (files: FileNode[]): string[] => {
  const selected: string[] = [];
  for (const file of files) {
    if (file.selected) {
      selected.push(file.path);
    }
    if (file.children) {
      selected.push(...getSelected(file.children));
    }
  }
  return selected;
};

export const useFileStore = create<FileStore>((set) => ({
  files: [],
  setFiles: (files) => {
    set({ files, selectedFiles: getSelected(files) });
  },
  selectedFiles: [],
  reset: () => {
    set({ files: [], selectedFiles: [] });
  },
}));
