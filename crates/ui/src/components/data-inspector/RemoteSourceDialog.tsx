import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Link as LinkIcon, Loader2, AlertCircle } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useHttp } from "@/hooks/useHttp";

interface RemoteSourceDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  urlInput: string;
  onUrlInputChange: (value: string) => void;
  onConnect: (url: string) => void;
}

export function RemoteSourceDialog({
  isOpen,
  onOpenChange,
  urlInput,
  onUrlInputChange,
  onConnect,
}: RemoteSourceDialogProps) {
  const { request, isLoading, error } = useHttp();

  const handleConnect = async () => {
    if (urlInput && urlInput.trim() && urlInput !== "https://") {
      try {
        await request(urlInput.trim());
        onConnect(urlInput.trim());
      } catch (err) {
        // Error is already handled by useHttp state
      }
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md bg-background border-border shadow-2xl">
        <DialogHeader className="space-y-3">
          <DialogTitle className="text-sm font-black uppercase tracking-[0.15em] flex items-center gap-3">
            <div className="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center">
              <LinkIcon className="h-4 w-4 text-primary" />
            </div>
            Remote Connection
          </DialogTitle>
          <DialogDescription className="text-xs font-medium text-muted-foreground ml-11">
            Establish a secure stream to external data endpoints.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-6 ml-11">
          <div className="space-y-2">
             <label htmlFor="remote-url" className="text-[10px] font-bold uppercase tracking-widest text-muted-foreground/70">Target Endpoint</label>
             <Input
              id="remote-url"
              value={urlInput}
              disabled={isLoading}
              onChange={(e) => onUrlInputChange(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && !isLoading && handleConnect()}
              className={`font-mono text-xs h-11 bg-secondary/30 border-border/60 focus-visible:ring-primary/20 ${error ? 'border-destructive focus-visible:ring-destructive' : ''}`}
              placeholder="s3://bucket/file.parquet"
              autoFocus
            />
          </div>

          {error && (
            <div className="flex items-start gap-2 p-3 bg-destructive/10 rounded-lg text-destructive animate-in fade-in slide-in-from-top-1 duration-300">
              <AlertCircle className="h-4 w-4 shrink-0 mt-0.5" />
              <p className="text-xs font-medium leading-relaxed">{error}</p>
            </div>
          )}
        </div>

        <DialogFooter className="sm:justify-end gap-2 sm:gap-0">
          <Button
            type="button"
            variant="secondary"
            disabled={isLoading}
            onClick={() => onOpenChange(false)}
            className="font-sans"
          >
            Cancel
          </Button>
          <Button
            type="button"
            onClick={handleConnect}
            disabled={isLoading}
            className="font-sans font-bold px-6"
          >
            {isLoading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Validating...
              </>
            ) : (
              "Connect"
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}