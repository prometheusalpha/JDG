import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar";
import { useFileStore } from "@/hooks/store";
import { search } from "@/lib/search";
import { Project } from "@/types/types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Plus, XIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { FileNode, FileTreeNode } from "./file-tree";
import { Input } from "./ui/input";

export function AppSidebar() {
  const [files, setFiles] = useState<FileNode[]>([]);

  const [selectedProject, setSelectedProject] = useState<string>("");
  const [projects, setProjects] = useState<Project[]>([]);

  const [keyword, setKeyword] = useState("");
  const { setFiles: setFileStore, reset } = useFileStore();

  const searchResult = useMemo(() => {
    if (!keyword) {
      return files;
    }
    return search(files, keyword);
  }, [files, keyword]);

  useEffect(() => {
    const fetchProjects = async () => {
      const projects: any = await invoke("get_projects");
      setProjects(projects);
      if (projects.length > 0) {
        const sortedProjects = projects.sort(
          (a: Project, b: Project) => b.last_opened - a.last_opened
        );
        // info("Sorted: " + JSON.stringify(sortedProjects));
        setSelectedProject(sortedProjects[0].id.toString());
      }
    };
    fetchProjects();
  }, []);

  useEffect(() => {
    if (!selectedProject) {
      return;
    }
    const fetchStructure = async () => {
      const structure: any = await invoke("read_file_structure", {
        id: selectedProject,
      });
      setFiles([structure]);
    };
    fetchStructure();
  }, [selectedProject]);

  const handleAddProject = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (!selected) {
        return;
      }

      const newPath = selected.replace(/\\/g, "/");

      // info(selected);
      const newProject = {
        id: projects.length + 1,
        name: newPath.split("/").pop() || "New Project",
        path: newPath,
        last_opened: new Date().getTime(),
      };
      setProjects([...projects, newProject]);
      await invoke("add_new_project", {
        id: newProject.id.toString(),
        name: newProject.name,
        path: newProject.path,
        last_opened: newProject.last_opened.toString(),
      });
      // info("Result: " + res);
    } catch (err) {
      console.error("Failed to open directory:", err);
    }
  };

  const handleSelect = (path: string, checked: boolean) => {
    const updateNode = (nodes: FileNode[], check: boolean): FileNode[] => {
      return nodes.map((node) => {
        if (node.path === path) {
          // When this is the target node, update it and all its children
          return {
            ...node,
            selected: check,
            children: node.children
              ? updateAllChildren(node.children, check)
              : undefined,
          };
        }
        if (node.children) {
          return {
            ...node,
            children: updateNode(node.children, check),
          };
        }
        return node;
      });
    };

    // Helper function to recursively update all children
    const updateAllChildren = (
      nodes: FileNode[],
      check: boolean
    ): FileNode[] => {
      return nodes.map((node) => ({
        ...node,
        selected: check,
        children: node.children
          ? updateAllChildren(node.children, check)
          : undefined,
      }));
    };

    setFiles((prevFiles) => {
      return updateNode(prevFiles, checked);
    });
    setFileStore(updateNode(files, checked));
  };

  const handleChangeProject = (id: string) => {
    setSelectedProject(id);
    reset();
  };

  return (
    <Sidebar className="">
      <SidebarContent>
        <SidebarHeader>
          <div className="flex flex-col gap-4">
            <div className="flex items-center gap-2">
              <Select
                value={selectedProject}
                onValueChange={handleChangeProject}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select a project" />
                </SelectTrigger>
                <SelectContent>
                  {projects.map((project) => (
                    <SelectItem key={project.id} value={project.id.toString()}>
                      {project.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <Button
                variant="outline"
                size="icon"
                onClick={handleAddProject}
                title="Add Project"
              >
                <Plus className="h-4 w-4" />
              </Button>
            </div>
          </div>
          <div className="relative">
            <Input
              placeholder="Search"
              value={keyword}
              onChange={(e) => setKeyword(e.target.value)}
              className="w-full focus-visible:ring-0"
            />
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className="absolute right-1 top-1/2 -translate-y-1/2 h-7 w-7 text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 cursor-pointer"
              onClick={() => {
                setKeyword("");
              }}
            >
              <XIcon className="h-4 w-4" />
              <span className="sr-only">Clear</span>
            </Button>
          </div>
        </SidebarHeader>
        <div className="mx-2 mb-10 overflow-auto">
          {searchResult &&
            searchResult.map((file) => (
              <FileTreeNode
                key={file.path}
                node={file}
                level={0}
                // onToggle={handleToggle}
                onSelect={handleSelect}
              />
            ))}
        </div>
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}
