import { Lock, Key, ShieldCheck, Database, EyeOff, Plus, Search, Trash2, Eye, LucideIcon, ShieldAlert } from "lucide-react";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";

interface VaultKey {
  id: string;
  label: string;
  provider: string;
  last_used: string;
  masked: string;
}

interface StatCardProps {
  label: string;
  value: string;
  icon: LucideIcon;
}

export function StatCard({ label, value, icon: Icon }: StatCardProps) {
  return (
    <div className="p-6 rounded-2xl border border-border/40 bg-secondary/5 flex items-center justify-between">
      <div className="space-y-1">
        <p className="text-[10px] font-bold text-muted-foreground uppercase tracking-[0.2em]">{label}</p>
        <p className="text-2xl font-black tracking-tighter">{value}</p>
      </div>
      <div className="h-12 w-12 rounded-xl bg-primary/5 border border-primary/10 flex items-center justify-center text-primary">
        <Icon className="h-6 w-6" />
      </div>
    </div>
  );
}

export default function VaultPage() {
  const [keys, setKeys] = useState<VaultKey[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [isAddOpen, setIsAddOpen] = useState(false);
  const [newName, setNewName] = useState("");
  const [newValue, setNewValue] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [revealedSecrets, setRevealedSecrets] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;
    const init = async () => {
      setIsLoading(true);
      try {
        const entries = await invoke<VaultKey[]>('get_vault_entries');
        if (isMounted) setKeys(entries);
      } catch (err) {
        toast.error("Hardware access failure", { description: String(err) });
      } finally {
        if (isMounted) setIsLoading(false);
      }
    };
    init();
    return () => { isMounted = false; };
  }, []);

  useEffect(() => {
    if (!isAddOpen) {
      setShowPassword(false);
      setNewName("");
      setNewValue("");
    }
  }, [isAddOpen]);

  const handleAddSecret = async () => {
    if (!newName || !newValue) {
      toast.error("Invalid Credentials");
      return;
    }
    try {
      const newEntry = await invoke<VaultKey>('save_secret', { label: newName, value: newValue, provider: "Manual" });
      setKeys([newEntry, ...keys]);
      setIsAddOpen(false);
      toast.success("Secret Encrypted");
    } catch (err) {
      toast.error("Encryption Failed", { description: String(err) });
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await invoke('delete_secret', { id });
      setKeys(keys.filter(k => k.id !== id));
      const nextRevealed = { ...revealedSecrets };
      delete nextRevealed[id];
      setRevealedSecrets(nextRevealed);
      toast.info("Key Destroyed");
    } catch (err) {
      toast.error("Destruction Failed", { description: String(err) });
    }
  };

  const performWipe = async () => {
    try {
      // In a real scenario, you might want a specific 'clear_vault' command
      // For now, we'll iterate or rely on the user manually deleting if the backend doesn't have a batch delete
      // However, to match History's 'purge', we should ideally have a 'purge_vault'
      for (const key of keys) {
        await invoke('delete_secret', { id: key.id });
      }
      setKeys([]);
      setRevealedSecrets({});
      toast.success("Vault Purged", { description: "All identity fragments have been destroyed." });
    } catch (err) {
      toast.error("Purge Failed", { description: String(err) });
    }
  };

  const toggleVisibility = async (id: string) => {
    if (revealedSecrets[id]) {
      const next = { ...revealedSecrets };
      delete next[id];
      setRevealedSecrets(next);
      return;
    }
    try {
      const plainText = await invoke<string>('reveal_secret', { id });
      setRevealedSecrets(prev => ({ ...prev, [id]: plainText }));
    } catch {
      toast.error("Decryption Denied");
    }
  };

  const filteredKeys = keys.filter(k => k.label.toLowerCase().includes(searchQuery.toLowerCase()));

  return (
    <div className="flex-1 flex flex-col bg-background overflow-hidden">
      <header className="h-20 border-b border-border/20 flex items-center px-8 justify-between shrink-0">
        <div>
          <h1 className="text-xl font-black tracking-tighter uppercase text-primary">Stronghold Vault</h1>
          <p className="text-[10px] text-muted-foreground uppercase tracking-widest font-bold opacity-60">
            {isLoading ? "Syncing Hardware..." : `${keys.length} Encrypted Identity Fragments`}
          </p>
        </div>
        <div className="flex items-center gap-4">
          <div className="relative w-64">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-3.5 w-3.5 text-muted-foreground pointer-events-none" />
            <Input 
              value={searchQuery} 
              onChange={(e) => setSearchQuery(e.target.value)} 
              className="pl-9 h-9 bg-transparent border-border text-xs font-mono placeholder:text-muted-foreground/50 focus-visible:ring-0 focus-visible:ring-offset-0" 
              placeholder="Filter identities..." 
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
            <AlertDialogContent className="bg-background border-border shadow-2xl">
              <AlertDialogHeader>
                <AlertDialogTitle className="font-black uppercase tracking-widest text-destructive">Wipe Protocol</AlertDialogTitle>
                <AlertDialogDescription className="text-xs">
                  This will permanently destroy all encrypted keys in the local vault. This action is terminal and cannot be undone.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel className="text-xs uppercase font-bold">Abort</AlertDialogCancel>
                <AlertDialogAction onClick={performWipe} className="bg-destructive text-destructive-foreground hover:bg-destructive/90 text-xs font-black uppercase">
                  Confirm Wipe
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>

          <Dialog open={isAddOpen} onOpenChange={setIsAddOpen}>
            <DialogTrigger asChild>
              <Button className="h-9 px-4 text-[10px] font-black uppercase tracking-widest gap-2">
                <Plus className="h-3 w-3" /> New Secret
              </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px] border-border bg-background">
              <DialogHeader>
                <DialogTitle className="text-sm font-black uppercase tracking-widest">Store New Secret</DialogTitle>
              </DialogHeader>
              <div className="grid gap-4 py-4">
                <div className="grid gap-2">
                  <Label className="text-[10px] uppercase font-bold text-muted-foreground">Identifier Name</Label>
                  <Input value={newName} onChange={(e) => setNewName(e.target.value)} placeholder="e.g. AWS Production" className="font-mono text-xs" />
                </div>
                <div className="grid gap-2">
                  <Label className="text-[10px] uppercase font-bold text-muted-foreground">Secret Value</Label>
                  <div className="relative">
                    <Input
                      type={showPassword ? "text" : "password"}
                      value={newValue}
                      onChange={(e) => setNewValue(e.target.value)}
                      placeholder="••••••••"
                      className="font-mono text-xs pr-10"
                    />
                    <button
                      type="button"
                      onClick={() => setShowPassword(!showPassword)}
                      className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                    >
                      {showPassword ? <EyeOff className="h-3.5 w-3.5" /> : <Eye className="h-3.5 w-3.5" />}
                    </button>
                  </div>
                </div>
              </div>
              <DialogFooter>
                <Button onClick={handleAddSecret} className="h-9 text-xs font-bold w-full sm:w-auto uppercase tracking-widest">Encrypt & Store</Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
      </header>

      <div className="flex-1 p-8 space-y-4 no-scrollbar pb-20 overflow-y-auto">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-4">
          <StatCard label="Protected Keys" value={keys.length.toString()} icon={Database} />
          <StatCard label="Security Layer" value="AES-256" icon={ShieldCheck} />
          <StatCard label="Hardware Status" value="ACTIVE" icon={Lock} />
        </div>

        {isLoading ? (
          <div className="h-64 flex flex-col items-center justify-center text-muted-foreground opacity-40 animate-pulse">
            <ShieldAlert className="h-12 w-12 mb-4" />
            <p className="text-xs font-bold uppercase tracking-widest">Accessing Secure Records...</p>
          </div>
        ) : filteredKeys.length === 0 ? (
          <div className="h-64 flex flex-col items-center justify-center text-muted-foreground opacity-40 select-none pb-20">
            <Key className="h-12 w-12 mb-4" />
            <p className="text-sm font-bold uppercase tracking-widest">Vault is Empty</p>
          </div>
        ) : (
          filteredKeys.map((key) => {
            const isRevealed = !!revealedSecrets[key.id];
            return (
              <div key={key.id} className="group flex items-center justify-between p-4 rounded-2xl border border-border/40 bg-secondary/5 hover:bg-secondary/10 transition-all duration-300">
                <div className="flex items-center gap-4">
                  <div className="h-10 w-10 rounded-xl bg-primary/5 border border-primary/10 flex items-center justify-center text-primary">
                    <Key className="h-5 w-5" />
                  </div>
                  <div className="flex flex-col">
                    <span className="text-sm font-bold tracking-tight">{key.label}</span>
                    <div className="flex items-center gap-3 mt-1">
                      <span className={`text-[10px] font-mono ${isRevealed ? 'text-primary font-black' : 'text-muted-foreground/40'}`}>
                        {isRevealed ? revealedSecrets[key.id] : key.masked}
                      </span>
                      <span className="text-[9px] text-muted-foreground/30 font-medium uppercase tracking-tighter">
                        {key.provider} • {key.last_used}
                      </span>
                    </div>
                  </div>
                </div>
                  <div className="flex items-center gap-2">
                    <Button variant="ghost" size="icon" onClick={() => toggleVisibility(key.id)} className="h-9 w-9 text-muted-foreground hover:text-primary transition-colors">
                      {isRevealed ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                    </Button>

                    <AlertDialog>
                      <AlertDialogTrigger asChild>
                        <Button variant="ghost" size="icon" className="h-9 w-9 text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors">
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </AlertDialogTrigger>
                      <AlertDialogContent className="bg-background border-border">
                        <AlertDialogHeader>
                          <AlertDialogTitle className="font-black uppercase tracking-widest text-destructive text-sm">Destroy Key</AlertDialogTitle>
                          <AlertDialogDescription className="text-xs">
                            Permanently delete "{key.label}"?
                          </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                          <AlertDialogCancel className="text-xs">Cancel</AlertDialogCancel>
                          <AlertDialogAction onClick={() => handleDelete(key.id)} className="bg-destructive text-destructive-foreground hover:bg-destructive/90 text-xs font-bold uppercase">
                            Confirm
                          </AlertDialogAction>
                        </AlertDialogFooter>
                      </AlertDialogContent>
                    </AlertDialog>
                  </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
