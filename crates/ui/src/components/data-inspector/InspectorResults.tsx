import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { AlertTriangle, CheckCircle, FileCode, X, Loader2, Globe, ExternalLink } from "lucide-react";
import { FileInspection } from "@/hooks/useFileInspector";
import { CodeBlock } from "../CodeBlock";
import { Alert, AlertDescription, AlertTitle } from "../ui/alert";
import { useFileSystem } from "@/hooks/useFileSystem";

interface InspectorResultsProps {
  result: FileInspection | null;
  filePath: string;
  onClear: () => void;
  onInspect: () => void;
  isLoading: boolean;
}

export function InspectorResults({ 
  result, 
  filePath, 
  onClear, 
  onInspect, 
  isLoading 
}: InspectorResultsProps) {
  const isRemote = filePath.startsWith('http') || filePath.startsWith('s3');
  const { showInFolder } = useFileSystem();

  return (
    <div className="flex-1 overflow-y-auto no-scrollbar py-8 px-6 animate-in fade-in duration-500">
      <div className="container mx-auto space-y-8 max-w-6xl flex flex-col items-center">
        <div className="w-full flex flex-col md:flex-row md:items-center justify-between gap-6 p-6 bg-secondary/20 rounded-2xl border border-border/40 shadow-sm backdrop-blur-sm">
          <div className="flex items-center gap-4 min-w-0">
            <div className="h-12 w-12 rounded-xl bg-primary/10 flex items-center justify-center shrink-0 shadow-inner">
              {isRemote ? <Globe className="h-6 w-6 text-primary" /> : <FileCode className="h-6 w-6 text-primary" />}
            </div>
            <div className="flex flex-col min-w-0">
              <span className="text-[10px] font-bold uppercase text-muted-foreground tracking-[0.2em] leading-none mb-1.5">
                {isRemote ? "Active Remote Stream" : "Analyzed Local Resource"}
              </span>
              <div className="flex items-center gap-2">
                <span className="font-mono text-sm truncate text-foreground font-medium max-w-[200px] md:max-w-md">{filePath}</span>
                {!isRemote && (
                  <button 
                    onClick={() => showInFolder(filePath)}
                    className="p-1 rounded hover:bg-primary/10 text-muted-foreground hover:text-primary transition-colors"
                    title="Show in File Explorer"
                  >
                    <ExternalLink className="h-3 w-3" />
                  </button>
                )}
              </div>
            </div>
          </div>

          <div className="flex items-center gap-3 shrink-0">
            {!result && (
              <Button 
                onClick={onInspect} 
                disabled={isLoading}
                size="default"
                className="font-bold shadow-lg shadow-primary/20 px-8 h-11"
              >
                {isLoading ? <Loader2 className="mr-2 h-4 w-4 animate-spin" /> : null}
                {isLoading ? "DIGESTING..." : "RUN ANALYSIS"}
              </Button>
            )}
            <Button 
              variant="outline" 
              size="icon" 
              onClick={onClear}
              className="h-11 w-11 rounded-xl border-border/60 text-muted-foreground hover:text-destructive hover:bg-destructive/5 transition-all"
              title="Close and Clear"
            >
              <X className="h-5 w-5" />
            </Button>
          </div>
        </div>

        {result && (
          <div className="w-full space-y-8">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <Card className="lg:col-span-2 border-border/60 shadow-sm overflow-hidden">
                <CardHeader className="py-4 px-6 border-b border-border/40 bg-secondary/10">
                  <CardTitle className="text-[10px] font-bold uppercase tracking-[0.2em] text-muted-foreground flex items-center gap-2">
                    <FileCode className="h-3.5 w-3.5" /> Metadata
                  </CardTitle>
                </CardHeader>
                <CardContent className="p-8">
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-10">
                    {[
                      { label: "Format", value: result.format, isMono: true },
                      { label: "Size", value: `${(result.size_bytes / 1024).toFixed(2)} KB`, isMono: true },
                      { label: "Density", value: result.metadata.information_density?.toFixed(4) || "N/A", isMono: true },
                      { label: "Depth", value: result.metadata.structural_depth || "Flat", isMono: true },
                    ].map((item) => (
                      <div key={item.label} className="space-y-2">
                        <p className="text-[10px] uppercase font-bold text-muted-foreground/50 tracking-widest">{item.label}</p>
                        <p className={`text-base font-semibold ${item.isMono ? 'font-mono' : 'font-sans'} text-foreground`}>
                          {item.value}
                        </p>
                      </div>
                    ))}
                  </div>
                </CardContent>
              </Card>

              <Card className="border-border/60 shadow-sm flex flex-col overflow-hidden">
                <CardHeader className="py-4 px-6 border-b border-border/40 bg-secondary/10">
                  <CardTitle className="text-[10px] font-bold uppercase tracking-[0.2em] text-muted-foreground">
                    Security Baseline
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-1 flex items-center p-8 bg-background">
                  {result.metadata.has_sensitive_data ? (
                    <Alert className={`max-w-md border-destructive/20 ${result.metadata.redacted ? 'bg-destructive/10 text-destructive' : 'bg-destructive/30 text-destructive animate-pulse'}`}>
                      <div className="gap-4 flex items-center">
                        <AlertTriangle />
                        <div>
                          <AlertTitle className="font-bold text-xs uppercase tracking-widest">
                            {result.metadata.redacted ? "PII Redacted" : "Compromised"}
                          </AlertTitle>
                          <AlertDescription className="text-[10px] opacity-80 mt-1">
                            {result.metadata.redacted 
                              ? "Sensitive data masked by policy." 
                              : "PII Exposed. Redaction is OFF."}
                          </AlertDescription>
                        </div>
                      </div>
                    </Alert>
                  ) : (
                    <Alert className="max-w-md bg-emerald-50 border border-emerald-400 text-emerald-700">
                      <div className="gap-4 flex items-center">
                        <CheckCircle />
                        <div>
                          <AlertTitle className="font-bold text-xs uppercase tracking-widest">Shield Active</AlertTitle>
                          <AlertDescription className="text-[10px] opacity-80 mt-1">No sensitive patterns found.</AlertDescription>
                        </div>
                      </div>
                    </Alert>
                  )}
                </CardContent>
              </Card>
            </div>

            <div className="space-y-4">
              <div className="flex items-center justify-between px-2">
                <h3 className="text-[10px] font-bold uppercase tracking-[0.2em] text-muted-foreground">
                  Structural Reality Visualization
                </h3>
                <div className="flex items-center gap-3">
                  <span className="text-[9px] font-bold font-mono bg-secondary px-2.5 py-1 rounded text-muted-foreground tracking-tighter">
                    ENCODING::UTF-8
                  </span>
                  <span className="text-[9px] font-bold font-mono bg-primary/10 text-primary px-2.5 py-1 rounded tracking-tighter">
                    SAMPLE_DEPTH::2000
                  </span>
                </div>
              </div>
              <CodeBlock 
                code={result.content_preview} 
                title={(() => {
                  const path = result.path.split('?')[0].replace(/\/$/, '');
                  const part = path.split('/').pop();
                  return part && part.length > 0 ? part : "remote_source";
                })()} 
                language={result.format.toLowerCase()} 
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
