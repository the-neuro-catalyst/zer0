import { FileCode, Search, Trash2, Clock, ShieldCheck } from "lucide-react";
import { useState, useEffect, useCallback } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

interface HistoryItem {
  path: string;
  format: string;
  scanned_at: string;
}

export default function HistoryPage() {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  const fetchHistory = useCallback(async () => {
    setIsLoading(true);
    try {
      if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        const data = await invoke<HistoryItem[]>("get_history");
        setHistory(data);
      } else {
        console.warn("Tauri invoke API not available in HistoryPage. Skipping history fetch.");
        setHistory([]);
      }
    } catch (err) {
      toast.error("Database Connection Failed", { description: String(err) });
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchHistory();
  }, [fetchHistory]);

  const filteredHistory = history.filter((item) =>
    item.path.toLowerCase().includes(searchQuery.toLowerCase()) ||
    item.format.toLowerCase().includes(searchQuery.toLowerCase())
  );

  const performClear = async () => {
    try {
      if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        await invoke("clear_history");
        setHistory([]);
        toast.success("Log Purged", { description: "All records have been destroyed." });
      } else {
        console.warn("Tauri invoke API not available for clearing history.");
        toast.error("Purge Failed", { description: "Tauri API not ready." });
      }
    } catch (err) {
      toast.error("Purge Failed", { description: String(err) });
    }
  };

  return (
    <div className="flex-1 flex flex-col overflow-hidden bg-background">
      <header className="h-20 border-b border-border/20 flex items-center px-8 justify-between shrink-0">
        <div>
          <h1 className="text-xl font-black tracking-tighter uppercase">History Log</h1>
          <p className="text-[10px] text-muted-foreground uppercase tracking-widest font-bold opacity-60">
            {isLoading ? "Synchronizing..." : `${history.length} Fragments Logged`}
          </p>
        </div>

        <div className="flex items-center gap-4">
          <div className="relative w-64">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground pointer-events-none" />
            <Input
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9 h-9 bg-transparent border-border text-xs font-mono placeholder:text-muted-foreground/50 focus-visible:ring-0 focus-visible:ring-offset-0"
              placeholder="Filter memory..."
            />
          </div>

          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="h-9 w-9 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent className="bg-background border-border">
              <AlertDialogHeader>
                <AlertDialogTitle className="font-black uppercase tracking-widest text-destructive">Destructive Action</AlertDialogTitle>
                <AlertDialogDescription className="text-xs">
                  This will permanently erase all local logs from the SQLite storage. This action cannot be reversed.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel className="text-xs">Abort</AlertDialogCancel>
                <AlertDialogAction onClick={performClear} className="bg-destructive text-destructive-foreground hover:bg-destructive/90 text-xs font-bold">
                  Confirm Purge
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </header>

      <div className="flex-1 p-8 space-y-4 no-scrollbar pb-20 overflow-y-auto">
        {isLoading ? (
          <div className="h-full flex flex-col items-center justify-center text-muted-foreground opacity-40 animate-pulse">
            <Clock className="h-12 w-12 mb-4" />
            <p className="text-xs font-bold uppercase tracking-widest">Accessing Secure Records...</p>
          </div>
        ) : filteredHistory.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-muted-foreground opacity-40 select-none pb-20">
            <Clock className="h-12 w-12 mb-4" />
            <p className="text-sm font-bold uppercase tracking-widest">No Logs Found</p>
          </div>
        ) : (
          filteredHistory.map((item, idx) => {
            const fileName = item.path.split('/').pop()?.split('\\').pop() || item.path;
            return (
              <div key={idx} className="group flex items-center justify-between p-4 rounded-2xl border border-border/40 bg-secondary/5 hover:bg-secondary/10 transition-all duration-300">
                <div className="flex items-center gap-4">
                  <div className="h-10 w-10 rounded-xl bg-primary/5 border border-primary/10 flex items-center justify-center text-primary">
                    <FileCode className="h-5 w-5" />
                  </div>
                  <div className="flex flex-col max-w-md">
                    <span className="text-sm font-bold tracking-tight truncate">{fileName}</span>
                    <div className="flex items-center gap-3 mt-1">
                      <span className="text-[9px] font-mono bg-secondary px-1.5 py-0.5 rounded text-muted-foreground uppercase">{item.format}</span>
                      <span className="text-[9px] text-muted-foreground/40 font-medium italic">
                        Detected on {new Date(item.scanned_at).toLocaleString()}
                      </span>
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <div className="flex items-center gap-1 text-emerald-500 bg-emerald-500/10 px-2 py-1 rounded-full">
                    <ShieldCheck className="h-3 w-3" />
                    <span className="text-[9px] font-black uppercase tracking-tighter">Verified</span>
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}