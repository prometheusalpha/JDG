import { Button } from "@/components/ui/button";
import { PanzoomObject } from "@panzoom/panzoom";
import { invoke } from "@tauri-apps/api/core";
import { writeImage } from "@tauri-apps/plugin-clipboard-manager";
import { info } from "@tauri-apps/plugin-log";
import { Buffer } from "buffer";
import { toPng } from "html-to-image";
import { ScanIcon, Settings2, ZoomIn, ZoomOut } from "lucide-react";
import { useEffect, useState } from "react";
import "./App.css";
import { CopyButton } from "./components/copy-button";
import { DebounceButton } from "./components/debounced-button";
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

function App() {
  const { selectedFiles } = useFileStore();
  const [mermaid, setMermaid] = useState("");
  const [vertical, setVertical] = useState(false);
  const [control, setControl] = useState<PanzoomObject | null>(null);

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

  const handleExportImage = async () => {
    // const state: State = {
    //   code: mermaid,
    //   mermaid: formatJSON({
    //     theme: "default",
    //   }),
    //   updateDiagram: true,
    //   rough: false,
    // };
    // const json = JSON.stringify(state);
    // const data = new TextEncoder().encode(json);
    // const compressed = pako.deflate(data, { level: 9 });
    // const encoded = fromUint8Array(compressed, true);
    // await open("https://mermaid.ink/img/pako:" + encoded);
    const dataUrl = await toPng(
      document.getElementById("randomId") as HTMLDivElement
    );
    const img = new Image();
    img.src = dataUrl;
    const res = await fetch(dataUrl);
    const blob = await res.blob();
    const reader = new FileReader();
    reader.readAsArrayBuffer(blob);
    reader.onloadend = async () => {
      const buffer = Buffer.from(reader.result as ArrayBuffer);
      await writeImage(buffer.buffer);
      // console.log("Copied to clipboard");
    };
  };

  const resetZoom = () => {
    control?.reset();
  };

  const zoomIn = () => {
    control?.zoomIn();
  };

  const zoomOut = () => {
    control?.zoomOut();
  };

  return (
    <Layout>
      <div className="flex w-screen h-screen bg-background">
        <div className="flex-1 flex flex-col">
          <div className="flex items-center justify-between p-4">
            <div className="flex gap-2">
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
              <Tabs defaultValue="diagram" className="h-full pb-10">
                <TabsList className="w-full">
                  <TabsTrigger value="diagram">Diagram</TabsTrigger>
                  <TabsTrigger value="source">Source</TabsTrigger>
                </TabsList>
                <TabsContent value="diagram">
                  <div className="flex justify-between items-center">
                    <div className="flex items-center py-2 gap-4">
                      <div className="flex items-center">
                        <Switch
                          checked={vertical}
                          onCheckedChange={setVertical}
                        />
                        <label className="ml-2">Rotate</label>
                      </div>
                      <DebounceButton
                        func={handleExportImage}
                        title="Copy PNG"
                      />
                    </div>
                    <div className="border bg-popover rounded-lg p-1">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={resetZoom}
                        disabled={!control}
                      >
                        <ScanIcon />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={zoomIn}
                        disabled={!control}
                      >
                        <ZoomIn />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={zoomOut}
                        disabled={!control}
                      >
                        <ZoomOut />
                      </Button>
                    </div>
                  </div>
                  <div className="border rounded overflow-hidden bg-neutral-100 h-full">
                    <Mermaid
                      chart={mermaid}
                      id={"randomId"}
                      onChange={setControl}
                    />
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
