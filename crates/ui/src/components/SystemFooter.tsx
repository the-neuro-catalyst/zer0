import { useEffect, useState } from "react";
import { Cpu, ShieldCheck, Zap } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface AppConfig {
  zero_copy: boolean;
  schema_inference: boolean;
  pii_redaction: boolean;
  strict_mode: boolean;
}

export function SystemFooter() {
  const [config, setConfig] = useState<AppConfig>({
    zero_copy: true,
    schema_inference: true,
    pii_redaction: true,
    strict_mode: false,
  });
  const [memory, setMemory] = useState<number>(0);

  useEffect(() => {
    async function loadStats() {
        try {
            const currentConfig = await invoke<AppConfig>('get_settings');
            setConfig(currentConfig);
            const mem = await invoke<number>('get_process_memory');
            setMemory(mem);
        } catch (err) {
            console.error("Failed to load system footer stats:", err);
        }
    }
    loadStats();
    const interval = setInterval(loadStats, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <footer className="h-8 border-t border-border/40 bg-background/80 backdrop-blur-sm flex items-center px-4 justify-between z-50 shrink-0 select-none">
      <div className="flex items-center gap-6">
        <div className="flex items-center gap-1.5 border-r border-border/40 pr-6">
          <Zap className={`h-3 w-3 ${config.zero_copy ? 'text-primary fill-primary/20' : 'text-muted-foreground/40'}`} />
          <span className="text-[9px] font-black text-foreground/70 uppercase tracking-widest">
            IO_STRATEGY::{config.zero_copy ? 'ZERO_COPY' : 'STANDARD'}
          </span>
        </div>
        <div className="flex items-center gap-4 text-muted-foreground/60">
          <div className="flex items-center gap-1.5">
            <Cpu className="h-2.5 w-2.5" />
            <span className="text-[8px] font-bold uppercase tracking-tighter">Memory_Footprint::</span>
            <span className="text-[9px] font-mono text-foreground/80">{memory.toFixed(1)} MB</span>
          </div>
        </div>
      </div>
      <div className="flex items-center gap-2">
        <ShieldCheck className={`h-3.5 w-3.5 ${config.strict_mode ? 'text-primary' : 'text-emerald-500/70'}`} />
        <span className="text-[9px] font-black text-foreground/50 uppercase tracking-[0.2em]">
          CRYPTOGRAPHY::AES_256_GCM
        </span>
      </div>
    </footer>
  );
}
