import { cn } from "@/lib/utils";
import { Copy, Check } from "lucide-react";
import { useState } from "react";
import { Card, CardContent, CardDescription, CardTitle } from "./ui/card";
import { Button } from "./ui/button";

interface CodeBlockProps {
  code: string;
  language?: string;
  title?: string;
  className?: string;
  copyable?: boolean;
}

export function CodeBlock({
  code,
  language = "json",
  title,
  className,
  copyable = true
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);
  const handleCopy = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <Card className={cn("rounded-lg border border-border bg-secondary/30 overflow-hidden", className)}>
      {title && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-border bg-secondary/50">
          <CardTitle className="font-mono text-[10px] tracking-wider text-muted-foreground">
            {title}
          </CardTitle>
          <CardDescription className="font-mono text-[10px] text-primary/60">{language}</CardDescription>
        </div>
      )}
      <CardContent className="relative p-0">
        <div className="overflow-x-auto p-4">
          <pre className="font-mono text-xs text-foreground/90 whitespace-pre">
            {code.split(/(\[REDACTED\])/g).map((part, index) => 
              part === "[REDACTED]" ? (
                <span key={index} className="text-destructive font-bold bg-destructive/10 px-1 rounded-sm">
                  {part}
                </span>
              ) : (
                part
              )
            )}
          </pre>
        </div>
        {copyable && (
          <Button
            size="sm"
            variant="outline"
            onClick={handleCopy}
            className="absolute top-2 right-2 rounded bg-secondary/80 hover:bg-secondary border border-border transition-colors"
          >
            {copied ? <Check color="#059669" /> : <Copy />}
          </Button>
        )}
      </CardContent>
    </Card>
  );
}
