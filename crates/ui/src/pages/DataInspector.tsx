import { useState, useEffect } from "react";
import { useFileInspector } from "@/hooks/useFileInspector";
// import { open } from '@tauri-apps/plugin-dialog'; // Removed direct import
import { listen } from '@tauri-apps/api/event';
import { InspectorDropZone } from "@/components/data-inspector/InspectorDropZone";
import { InspectorResults } from "@/components/data-inspector/InspectorResults";
import { RemoteSourceDialog } from "@/components/data-inspector/RemoteSourceDialog";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";

export default function DataInspector() {
  const [filePath, setFilePath] = useState("");
  const [isDragging, setIsDragging] = useState(false);
  const [isUrlDialogOpen, setIsUrlDialogOpen] = useState(false);
  const [urlInput, setUrlInput] = useState("https://");
  const { inspect, isLoading, result, reset } = useFileInspector();

  useEffect(() => {
    let unlistenDrop: Function | undefined;
    let unlistenEnter: Function | undefined;
    let unlistenLeave: Function | undefined;

    // Guard against Tauri API not being ready before setting up listeners
    // Use `typeof invoke === 'function'` as a robust check for API readiness
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof listen === 'function') {
        console.log("Tauri API seems ready in DataInspector. Setting up listeners.");
        // Tauri specific file drop listeners
        listen('tauri://drag-drop', (event) => {
            setIsDragging(false);
            const payload = event.payload as { paths: string[] };
            if (payload.paths && payload.paths.length > 0) {
                const path = payload.paths[0];
                setFilePath(path);
                const fileName = path.split(/[/\\]/).pop() || path;
                toast.success("Resource detected", {
                    description: `Ready to analyze: ${fileName}`
                });
            }
        }).then(unsub => unlistenDrop = unsub); // Capture the unlisten function

        listen('tauri://drag-enter', () => setIsDragging(true)).then(unsub => unlistenEnter = unsub);
        listen('tauri://drag-leave', () => setIsDragging(false)).then(unsub => unlistenLeave = unsub);
    } else {
        console.warn("Tauri API not fully initialized yet in DataInspector. Skipping event listeners setup.");
    }

    return () => {
      // Ensure unlisten functions are defined before calling them and unlisten
      unlistenDrop?.();
      unlistenEnter?.();
      unlistenLeave?.();
    };
  }, []);

  const handleInspect = async (pathOverride?: string) => {
    const targetPath = pathOverride || filePath;
    if (targetPath) {
      // Guard invoke calls
      if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        try {
            // Call inspect with a default head value (e.g., 100 rows)
            const res = await inspect(targetPath, 100); 
            if (res) {
                // Ensure invoke is ready before calling it
                // It's already checked by the outer if, but good to be explicit
                invoke("add_to_history", { path: res.path, format: res.format });
                toast.success("Analysis Complete", {
                    description: "Data structure mapped successfully."
                });
            }
        } catch (err) {
            toast.error("Analysis Failed", {
              description: err instanceof Error ? err.message : "Internal engine error."
            });
        }
      } else {
          console.error("Tauri invoke API not available for inspection.");
          toast.error("Analysis Failed", {
              description: "Tauri API not ready."
          });
      }
    }
  };

  const handleFileSelect = async () => {
    // Dynamically import 'open' and guard its usage
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        try {
            // Dynamic import to ensure it's loaded late when invoke is available
            const { open } = await import('@tauri-apps/plugin-dialog');
            const selected = await open({ multiple: false, directory: false });
            if (selected && typeof selected === 'string') {
                setFilePath(selected);
                toast.info("Source selected via local dialog");
                // Trigger inspection immediately for better UX
                handleInspect(selected);
            }
        } catch (err) {
            console.error("File selection error:", err);
            toast.error("File Selection Failed", { description: String(err) });
        }
    } else {
        console.error("Tauri API not available for file selection.");
        toast.error("File Selection Failed", { description: "Tauri API not ready." });
    }
  };

  const onRemoteConnectSuccess = (url: string) => {
    setFilePath(url);
    setIsUrlDialogOpen(false);
    // Automatically trigger inspection after remote validation
    handleInspect(url);
  };

  const handleClear = () => {
    setFilePath("");
    reset();
    toast.info("Workspace Cleared", {
      description: "Working memory cleared."
    });
  };

  return (
    <>
      {/* Main Content Slot */}
      <div className="flex-1 flex flex-col relative">
        {isLoading ? ( // Check isLoading here
          <div className="absolute inset-0 flex items-center justify-center bg-background/80 z-10 text-lg font-semibold">
            Loading...
          </div>
        ) : !filePath ? (
          <InspectorDropZone
            isDragging={isDragging}
            onFileSelect={handleFileSelect}
            onRemoteConnect={() => setIsUrlDialogOpen(true)}
          />
        ) : (
          <InspectorResults
            result={result}
            filePath={filePath}
            onClear={handleClear}
            onInspect={() => handleInspect()}
            isLoading={isLoading}
          />
        )}
      </div>

      {/* Dialogs */}
      <RemoteSourceDialog
        isOpen={isUrlDialogOpen}
        onOpenChange={setIsUrlDialogOpen}
        urlInput={urlInput}
        onUrlInputChange={setUrlInput}
        onConnect={onRemoteConnectSuccess}
      />
    </>
  );
}