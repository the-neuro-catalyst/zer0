import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

export interface FileInspection {
  path: string;
  format: string;
  size_bytes: number;
  content_preview: string;
  metadata: {
    line_count?: number;
    information_density?: number;
    structural_depth?: number;
    has_sensitive_data: boolean;
    redacted: boolean;
  };
}

export function useFileInspector() {
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<FileInspection | null>(null);
  const [error, setError] = useState<string | null>(null);

  const inspect = async (path: string, head?: number) => { // Added head parameter
    if (!path.trim()) { toast.error("File path cannot be empty"); return; }
    setIsLoading(true);
    setError(null);
    try {
      // Pass head to the invoke call
      const data = await invoke<FileInspection>("inspect_file", { path, head }); 
      setResult(data);
      return data;
    } catch (err) {
      const message = String(err);
      setError(message);
      toast.error(`Inspection failed: ${message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const reset = () => { setResult(null); setError(null); };

  return { inspect, reset, isLoading, result, error };
}
