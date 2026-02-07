import { Button } from "@/components/ui/button";
import { FileUp, FolderOpen, Globe } from "lucide-react";
import { Card, CardDescription, CardTitle } from "../ui/card";

interface InspectorDropZoneProps {
  isDragging: boolean;
  onFileSelect: () => void;
  onRemoteConnect: () => void;
}

export function InspectorDropZone({
  isDragging,
  onFileSelect,
  onRemoteConnect,
}: InspectorDropZoneProps) {
  return (
    <div className="flex-1 flex flex-col items-center justify-center p-6 animate-in fade-in duration-500">
      <div className="text-center space-y-2 mb-12">
        <h1 className="text-6xl md:text-8xl font-black tracking-[0.15em] text-foreground uppercase">
          ZERO
        </h1>
        <p className="text-muted-foreground text-xs font-bold uppercase tracking-[0.4em] max-w-2xl mx-auto opacity-70">
          Universal Data Inspector
        </p>
      </div>

      <Card
        className={`w-full max-w-4xl aspect-[21/9] rounded-2xl border-2 border-dashed transition-all duration-300 flex flex-col items-center justify-center gap-6 ${isDragging
          ? "border-primary bg-primary/5 scale-[1.01] shadow-2xl shadow-primary/10 ring-8 ring-primary/5"
          : "border-border/60 bg-card hover:border-primary/30"
          }`}
      >
        <div className={`w-20 h-20 rounded-2xl bg-secondary flex items-center justify-center shadow-inner transition-transform duration-300 ${isDragging ? "scale-110" : ""}`}>
          <FileUp className={`h-10 w-10 text-muted-foreground transition-colors ${isDragging ? "text-primary" : ""}`} />
        </div>

        <div className="text-center space-y-1">
          <CardTitle className="text-xl font-semibold text-foreground">
            {isDragging ? "Release to Analyze" : "Drag & Drop File Here"}
          </CardTitle>
          <CardDescription className="text-sm text-muted-foreground">or choose a method below</CardDescription>
        </div>

        <div className="flex items-center gap-4 mt-2">
          <Button onClick={onFileSelect} variant="outline" className="font-semibold px-6 border-primary/20 hover:bg-primary/5 hover:text-primary">
            <FolderOpen className="mr-2 h-4 w-4" /> Browse Local
          </Button>
          <div className="w-px h-8 bg-border/60" />
          <Button onClick={onRemoteConnect} variant="ghost" className="text-muted-foreground hover:text-primary hover:bg-primary/5">
            <Globe className="mr-2 h-4 w-4" /> Remote URL
          </Button>
        </div>
      </Card>

      <div className="mt-12 max-w-2xl text-center text-[10px] font-bold tracking-[0.2em] text-muted-foreground/30 uppercase leading-loose">
        <span>PARQUET &bull; CSV &bull; JSON &bull; SQLITE &bull; PDF &bull; S3 &bull; KAFKA &bull; POSTGRES</span>
      </div>
    </div>
  );
}
