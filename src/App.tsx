import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import { Settings2 } from "lucide-react";
import { useEffect, useState } from "react";
import "./App.css";
import { CopyButton } from "./components/copy-button";
import Mermaid from "./components/mermaid";
import { ModeToggle } from "./components/mode-toggle";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "./components/ui/dialog";
import { Switch } from "./components/ui/switch";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./components/ui/tabs";
import { useFileStore } from "./hooks/store";
import Layout from "./layout";
// import pako from "pako";

function App() {
  const { selectedFiles } = useFileStore();
  const [mermaid, setMermaid] = useState("");
  const [vertical, setVertical] = useState(false);

  useEffect(() => {
    // info("Generating mermaid class diagram");
    if (selectedFiles.length === 0) {
      setMermaid("");
      return;
    }
    invoke("generate_mermaid_class_diagram", {
      filePaths: Array.from(selectedFiles),
      vertical,
    })
      .then((res) => {
        // info(JSON.stringify(res, null, 2));
        setMermaid(res as string);
      })
      .catch((e) => {
        info(e);
      });
  }, [selectedFiles, vertical]);

  // const handleCopyPng = async () => {
  //   const data = btoa(mermaid);
  //   await open("https://mermaid.live/edit#base64:" + data);
  // };

  return (
    <Layout>
      <div className="flex w-screen h-screen bg-background">
        <div className="flex-1 flex flex-col">
          <div className="flex items-center justify-between p-4">
            <div className="flex gap-2">
              {/* <Button variant="outline" size="sm">
                <Download className="h-4 w-4 mr-2" />
                Export
              </Button> */}
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="outline" size="sm">
                    <Settings2 className="h-4 w-4 mr-2" />
                    Settings
                  </Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Settings</DialogTitle>
                  </DialogHeader>
                  <div className="">
                    <div className="flex justify-between">
                      <p>Dark mode</p>
                      <ModeToggle />
                    </div>
                  </div>
                </DialogContent>
              </Dialog>
            </div>
          </div>
          <div className="flex-1 p-4">
            {mermaid ? (
              <Tabs defaultValue="diagram" className="">
                <TabsList className="w-full">
                  <TabsTrigger value="diagram">Diagram</TabsTrigger>
                  <TabsTrigger value="source">Source</TabsTrigger>
                </TabsList>
                <TabsContent value="diagram">
                  {/* <img
                    src={`https://mermaid.ink/img/${btoa(mermaid)}`}
                    alt="Mermaid Diagram"
                  /> */}
                  <div className="flex items-center py-2 gap-4">
                    <div className="flex items-center">
                      <Switch
                        checked={vertical}
                        onCheckedChange={setVertical}
                      />
                      <label className="ml-2">Vertical</label>
                    </div>
                    {/* <Button variant="outline" size="sm" onClick={handleCopyPng}>
                      Copy PNG
                    </Button> */}
                  </div>
                  <div className="border rounded overflow-hidden bg-neutral-100">
                    <Mermaid chart={mermaid} id={"mermaid"} />
                  </div>
                </TabsContent>
                <TabsContent value="source">
                  <div className="border rounded w-full">
                    <div className="border-b p-2 flex justify-end">
                      <CopyButton value={mermaid} />
                    </div>
                    <div className="p-4">
                      <pre>
                        <code>{mermaid}</code>
                      </pre>
                    </div>
                  </div>
                </TabsContent>
              </Tabs>
            ) : (
              <div className="flex items-center justify-center h-full text-muted-foreground">
                Select a project and files to generate class diagram
              </div>
            )}
          </div>
        </div>
      </div>
    </Layout>
  );
}

export default App;
