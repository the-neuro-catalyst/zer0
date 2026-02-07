import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus, Square, Copy } from "lucide-react";
import { useEffect, useState, useMemo } from "react";
import { Button } from "@/components/ui/button";
import { useTheme } from "next-themes";

export function WindowManager() {
  const [isMaximized, setIsMaximized] = useState(false);
  const { resolvedTheme } = useTheme();

  // Conditionally initialize appWindow only if Tauri API is available
  const appWindow = useMemo(() => {
    // Check if window.__TAURI_INTERNALS__ exists before calling getCurrentWindow
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__) {
      try {
        return getCurrentWindow();
      } catch (e) {
        console.error("Error getting current window from Tauri API:", e);
        return null; // Return null if getCurrentWindow fails
      }
    }
    return null; // Return null if Tauri internals are not available
  }, []); // Re-evaluate only once on mount

  useEffect(() => {
    // Only proceed if appWindow is not null
    if (appWindow) {
      const updateMaximized = async () => {
        try {
          setIsMaximized(await appWindow.isMaximized());
        } catch (error: unknown) {
          console.error("Failed to get window state:", error);
        }
      };
      updateMaximized();
      const unlisten = appWindow.onResized(() => {
        updateMaximized();
      });
      return () => {
        unlisten.then(f => f());
      };
    }
  }, [appWindow]);

  const handleDrag = async () => {
    if (appWindow) { // Check if appWindow is available
      try {
        await appWindow.startDragging();
      } catch (error: unknown) {
        console.error("Failed to start window drag:", error);
      }
    }
  };

  const handleDoubleClick = async () => {
    if (appWindow) { // Check if appWindow is available
      await appWindow.toggleMaximize();
    }
  };

  const logoSrc = resolvedTheme === "dark" ? "ZERO-WHITE.png" : "ZERO-BLACK.png";

  return (
    <div
      data-tauri-drag-region
      onMouseDown={handleDrag}
      onDoubleClick={handleDoubleClick}
      className="h-10 w-full bg-background flex items-center justify-between px-4 shrink-0 select-none z-[100] border-b border-border/20 cursor-default"
    >
      <div className="flex items-center gap-2 pointer-events-none">
        <div className="h-4 w-4 rounded flex items-center justify-center">
          <img src={logoSrc} alt="App Icon" className="h-4 w-4" />
        </div>
        <span className="text-[10px] font-black uppercase tracking-[0.2em] text-black/80 dark:text-white/80">
          ZERO
        </span>
      </div>

      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={() => appWindow?.minimize()} // Use optional chaining
          className="h-8 w-8 text-muted-foreground hover:bg-secondary"
          title="Minimize"
        >
          <Minus className="h-3.5 w-3.5" />
        </Button>

        <Button
          variant="ghost"
          size="icon"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={() => appWindow?.toggleMaximize()} // Use optional chaining
          className="h-8 w-8 text-muted-foreground hover:bg-secondary"
          title={isMaximized ? "Restore" : "Maximize"}
        >
          {isMaximized ? <Copy className="h-3 w-3" /> : <Square className="h-3 w-3" />}
        </Button>

        <Button
          variant="ghost"
          size="icon"
          onMouseDown={(e) => e.stopPropagation()}
          onClick={() => appWindow?.close()} // Use optional chaining
          className="h-8 w-8 text-muted-foreground hover:bg-destructive hover:bg-destructive/90 hover:text-destructive-foreground"
          title="Close"
        >
          <X className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}
