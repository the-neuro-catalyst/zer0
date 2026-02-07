import { Zap, Shield, Cpu, LucideIcon, Palette, Trash2, AlertTriangle, LogOut } from "lucide-react";
import { ThemeToggle } from "@/components/ThemeToggle";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";
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
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { exit } from "@tauri-apps/plugin-process";
import { toast } from "sonner";

interface SettingRowProps {
  icon: LucideIcon;
  label: string;
  description: string;
  children: React.ReactNode;
}

function SettingRow({ icon: Icon, label, description, children }: SettingRowProps) {
  return (
    <div className="flex items-center justify-between p-6 rounded-2xl border border-border/40 bg-secondary/5">
      <div className="flex items-center gap-4">
        <div className="h-10 w-10 rounded-xl bg-background border border-border/40 flex items-center justify-center text-muted-foreground">
          <Icon className="h-5 w-5" />
        </div>
        <div>
          <p className="text-sm font-bold tracking-tight">{label}</p>
          <p className="text-[10px] text-muted-foreground font-medium uppercase tracking-tighter opacity-60 mt-0.5">{description}</p>
        </div>
      </div>
      <div>{children}</div>
    </div>
  );
}

interface AppConfig {
  zero_copy: boolean;
  schema_inference: boolean;
  pii_redaction: boolean;
  strict_mode: boolean;
}

export default function SettingsPage() {
  const [config, setConfig] = useState<AppConfig>({
    zero_copy: true,
    schema_inference: true,
    pii_redaction: true,
    strict_mode: false,
  });

  useEffect(() => {
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        invoke<AppConfig>('get_settings')
          .then(setConfig)
          .catch(err => console.error("Failed to load settings:", err));
    } else {
        console.warn("Tauri API not fully initialized in SettingsPage. Skipping settings load.");
    }
  }, []);

  const handleToggle = (key: keyof AppConfig, value: boolean) => {
    setConfig(prev => ({ ...prev, [key]: value }));
    
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
        invoke('update_setting', { key, value })
          .catch(err => {
            console.error("Failed to update setting:", err);
            setConfig(prev => ({ ...prev, [key]: !value }));
            toast.error("Settings Sync Failed");
          });
    } else {
        console.warn("Tauri API not fully initialized in SettingsPage. Cannot update setting.");
        toast.error("Settings Sync Failed", { description: "Tauri API not ready." });
    }
  };

  const handleGlobalPurge = async () => {
    if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function' && typeof exit === 'function') {
        try {
            await invoke('purge_all_data');
            toast.success("Identity Neutralized", { description: "All local data destroyed. System exiting." });
            setTimeout(async () => {
                await exit(0);
            }, 2000);
        } catch (err) {
            toast.error("Purge Protocol Failed", { description: String(err) });
        }
    } else {
        console.error("Tauri API not available for global purge.");
        toast.error("Purge Protocol Failed", { description: "Tauri API not ready." });
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-background overflow-hidden">
      <header className="h-20 border-b border-border/20 flex items-center px-8 shrink-0">
        <div>
          <h1 className="text-xl font-black tracking-tighter uppercase">Engine Settings</h1>
          <p className="text-[10px] text-muted-foreground uppercase tracking-widest font-bold opacity-60">Fine-tuning ZERO ENGINE</p>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-8 space-y-10 no-scrollbar pb-20">
        <section className="space-y-4">
          <h2 className="text-xs font-bold uppercase tracking-[0.3em] text-muted-foreground flex items-center gap-2">
            <Palette className="h-3 w-3" /> Visual Interface
          </h2>
          <div className="space-y-3">
            <SettingRow icon={Palette} label="System Theme" description="Toggle between Light, Dark, or System preference">
              <ThemeToggle />
            </SettingRow>
          </div>
        </section>

        <section className="space-y-4">
          <h2 className="text-xs font-bold uppercase tracking-[0.3em] text-primary flex items-center gap-2">
            <Zap className="h-3 w-3 fill-primary" /> Core Engine
          </h2>
          <div className="space-y-3">
            <SettingRow icon={Cpu} label="Zero-Copy Pipeline" description="Maximize data processing speed via memory mapping">
              <Switch checked={config.zero_copy} onCheckedChange={(v) => handleToggle('zero_copy', v)} />
            </SettingRow>
            <SettingRow icon={Zap} label="Schema Auto-Inference" description="Automatically detect structure from raw binary streams">
              <Switch checked={config.schema_inference} onCheckedChange={(v) => handleToggle('schema_inference', v)} />
            </SettingRow>
          </div>
        </section>

        <section className="space-y-4">
          <h2 className="text-xs font-bold uppercase tracking-[0.3em] text-muted-foreground flex items-center gap-2">
            <Shield className="h-3 w-3" /> Defensive Protocols
          </h2>
          <div className="space-y-3">
            <SettingRow icon={Shield} label="PII Redaction" description="Automatically hide sensitive data in previews">
              <Switch checked={config.pii_redaction} onCheckedChange={(v) => handleToggle('pii_redaction', v)} />
            </SettingRow>
            <SettingRow icon={Shield} label="Strict Content Policy" description="Block non-validated remote endpoints">
              <Switch checked={config.strict_mode} onCheckedChange={(v) => handleToggle('strict_mode', v)} />
            </SettingRow>
          </div>
        </section>

        <section className="pt-10 border-t border-border/20">
            <div className="p-8 rounded-2xl border border-destructive/20 bg-destructive/5 space-y-6">
                <div className="flex items-start gap-4">
                    <div className="h-12 w-12 rounded-xl bg-destructive/10 flex items-center justify-center text-destructive shrink-0">
                        <AlertTriangle className="h-6 w-6" />
                    </div>
                    <div>
                        <h3 className="text-lg font-black uppercase tracking-tighter text-destructive">Nuclear Exit Strategy</h3>
                        <p className="text-xs text-muted-foreground font-medium leading-relaxed max-w-xl mt-1">
                            Executing this protocol will permanently destroy all local data <b className="text-destructive">associated with ZERO</b>, 
                            including history, encrypted vault secrets, and master keys. <b className="text-destructive">Your original source files remain untouched.</b> 
                            This action is terminal and cannot be recovered.
                        </p>
                    </div>
                </div>

                <div className="flex items-center gap-4">
                    <AlertDialog>
                        <AlertDialogTrigger asChild>
                            <Button variant="destructive" className="font-black uppercase tracking-widest text-[10px] h-11 px-8 gap-2">
                                <Trash2 className="h-4 w-4" /> Purge All Data
                            </Button>
                        </AlertDialogTrigger>
                        <AlertDialogContent className="bg-background border-destructive/50">
                            <AlertDialogHeader>
                                <AlertDialogTitle className="text-destructive font-black uppercase tracking-widest">Global Data Destruction</AlertDialogTitle>
                                <AlertDialogDescription className="text-xs">
                                    Are you absolutely certain? This will wipe your history and ALL encrypted secrets. 
                                    The application will terminate immediately after the purge.
                                </AlertDialogDescription>
                            </AlertDialogHeader>
                            <AlertDialogFooter>
                                <AlertDialogCancel className="text-xs">Abort</AlertDialogCancel>
                                <AlertDialogAction onClick={handleGlobalPurge} className="bg-destructive text-destructive-foreground hover:bg-destructive/90 text-xs font-black uppercase">
                                    Confirm Destruction
                                </AlertDialogAction>
                            </AlertDialogFooter>
                        </AlertDialogContent>
                    </AlertDialog>

                    <Button variant="outline" onClick={() => {
                        if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof exit === 'function') {
                            exit(0);
                        } else {
                            console.warn("Tauri API not fully initialized. Cannot exit gracefully.");
                            toast.error("Exit Failed", { description: "Tauri API not ready." });
                        }
                    }} className="font-bold uppercase tracking-widest text-[10px] h-11 px-8 gap-2">
                        <LogOut className="h-4 w-4" /> Graceful Exit
                    </Button>
                </div>
            </div>
        </section>
      </div>
    </div>
  );
}
