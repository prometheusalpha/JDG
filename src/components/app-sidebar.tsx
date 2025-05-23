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
import { Project } from "@/types/types";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { info } from "@tauri-apps/plugin-log";
import { Plus } from "lucide-react";
import { useEffect, useState } from "react";
import { FileNode, FileTreeNode } from "./file-tree";

export function AppSidebar() {
  const [files, setFiles] = useState<FileNode[]>([]);

  const [selectedProject, setSelectedProject] = useState<string>("");
  const [projects, setProjects] = useState<Project[]>([]);

  const { setFiles: setFileStore } = useFileStore();

  useEffect(() => {
    const fetchProjects = async () => {
      const projects: any = await invoke("get_projects");
      setProjects(projects);
      if (projects.length > 0) {
        setSelectedProject(projects[0].id.toString());
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
      // info(selected);
      const newProject = {
        id: projects.length + 1,
        name: selected.split("/").pop() || "New Project",
        path: selected,
        lastOpened: new Date().getTime(),
      };
      setProjects([...projects, newProject]);
      let res = await invoke("add_new_project", {
        id: newProject.id.toString(),
        name: newProject.name,
        path: newProject.path,
        lastOpened: newProject.lastOpened.toString(),
      });
      info("Result: " + res);
    } catch (err) {
      console.error("Failed to open directory:", err);
    }
  };

  // const handleToggle = (path: string) => {};

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

  return (
    <Sidebar className="">
      <SidebarContent>
        <SidebarHeader>
          <div className="flex flex-col gap-4">
            <div className="flex items-center gap-2">
              <Select
                value={selectedProject}
                onValueChange={setSelectedProject}
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
        </SidebarHeader>
        <div className="mx-2 mb-10">
          {files && files.map((file) => (
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
