import { useState, useEffect } from "react";
import { BookOpen, Info, Globe } from "lucide-react";
import { Button } from "@/components/ui/button";
import { openUrl } from '@tauri-apps/plugin-opener';
import { getVersion, getName } from '@tauri-apps/api/app';
import { toast } from "sonner";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog";
import { cn } from "@/lib/utils";
import { useTheme } from "next-themes";
import { invoke } from '@tauri-apps/api/core';

// --- Brand Icons ---
const GithubIcon = ({ className }: { className?: string }) => (
    <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" className={className}><title>GitHub</title><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" /></svg>
);

const DiscordIcon = ({ className }: { className?: string }) => (
    <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" className={className}><title>Discord</title><path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.7748-.6091 1.1696-1.8703-.279-3.7346-.279-5.555 0-.17-.4026-.4114-.8093-.6344-1.1895a.077.077 0 00-.0793-.0376 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.419-2.1568 2.419zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.419-2.1568 2.419z" /></svg>
);

const XIcon = ({ className }: { className?: string }) => (
    <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" className={className}><title>X</title><path d="M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z" /></svg>
);

const SlackIcon = ({ className }: { className?: string }) => (
    <svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor" className={className}><title>Slack</title><path d="M5.042 15.165a2.528 2.528 0 0 1-2.52 2.523A2.528 2.528 0 0 1 0 15.165a2.527 2.527 0 0 1 2.522-2.52h2.52v2.52zM6.313 15.165a2.527 2.527 0 0 1 2.521-2.52 2.527 2.527 0 0 1 2.521 2.52v6.313A2.528 2.528 0 0 1 8.834 24a2.528 2.528 0 0 1-2.521-2.52v-6.315zM8.834 5.042a2.528 2.528 0 0 1-2.521-2.52A2.528 2.528 0 0 1 8.834 0a2.528 2.528 0 0 1 2.521 2.522v2.52h-2.521zM8.834 6.313a2.528 2.528 0 0 1 2.521 2.521 2.528 2.528 0 0 1-2.521 2.521H2.522A2.528 2.528 0 0 1 0 8.834a2.528 2.528 0 0 1 2.522-2.521h6.312zM18.956 8.834a2.528 2.528 0 0 1 2.522-2.521A2.528 2.528 0 0 1 24 8.834a2.528 2.528 0 0 1-2.522 2.521h-2.522V8.834zM17.688 8.834a2.528 2.528 0 0 1-2.523 2.521 2.527 2.527 0 0 1-2.52-2.521V2.522A2.527 2.527 0 0 1 15.165 0a2.528 2.528 0 0 1 2.523 2.522v6.312zM15.165 18.956a2.528 2.528 0 0 1 2.523 2.522A2.528 2.528 0 0 1 15.165 24a2.527 2.527 0 0 1-2.52-2.522v-2.522h2.52zM15.165 17.688a2.527 2.527 0 0 1-2.52-2.523 2.526 2.526 0 0 1 2.52-2.52h6.313A2.527 2.527 0 0 1 24 15.165a2.528 2.528 0 0 1-2.522 2.523h-6.313z" /></svg>
);

interface SidebarActionsProps {
    isCollapsed: boolean;
}

export function SidebarActions({ isCollapsed }: SidebarActionsProps) {
    const [appVersion, setAppVersion] = useState("Unknown");
    const [appName, setAppName] = useState("ZERO");
    const { resolvedTheme } = useTheme();

    useEffect(() => {
        async function loadMeta() {
            if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
                try {
                    setAppVersion(await getVersion());
                    setAppName(await getName());
                } catch (e) {
                    console.error("Failed to load app metadata:", e);
                    setAppVersion("N/A");
                    setAppName("ZERO");
                }
            } else {
                console.warn("Tauri API not fully initialized yet in SidebarActions. Skipping metadata load.");
                setAppVersion("N/A");
                setAppName("ZERO");
            }
        }
        loadMeta();
    }, []);

    const openLink = async (url: string) => {
        if (typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__ && typeof invoke === 'function') {
            try {
                await openUrl(url);
            } catch {
                toast.error("Navigation Failed", { description: "Could not open browser." });
            }
        } else {
            console.warn("Tauri API not fully initialized yet. Cannot open external URL.");
            toast.error("Navigation Failed", { description: "Tauri API not ready to open browser." });
        }
    };

    const logoSrc = resolvedTheme === "dark" ? "ZERO-WHITE.png" : "ZERO-BLACK.png";

    return (
        <div className="flex flex-col gap-6 px-2.5 mb-4 mt-auto">
            {/* Category: System */}
            <div className="space-y-1">
                {!isCollapsed && <p className="px-3 text-[9px] font-black uppercase tracking-widest text-muted-foreground/50 mb-2">System</p>}

                <Dialog>
                    <DialogTrigger asChild>
                        <Button
                            variant="ghost"
                            className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                        >
                            <Info className="h-4 w-4 shrink-0 group-hover:text-primary transition-colors" />
                            {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">About</span>}
                        </Button>
                    </DialogTrigger>
                    <DialogContent className="sm:max-w-[400px] border-border bg-background pb-12">
                        <DialogHeader className="flex flex-col items-center text-center space-y-4 pt-4">
                            <div className="h-16 w-16 rounded-2xl flex items-center justify-center">
                                <img src={logoSrc} alt="ZERO" className="h-12 w-12" />
                            </div>
                            <div className="space-y-1 text-center mx-auto">
                                <DialogTitle className="text-xl font-black tracking-[0.2em] uppercase">{appName}</DialogTitle>
                                <DialogDescription className="font-mono text-xs text-muted-foreground">
                                    v{appVersion}
                                </DialogDescription>
                            </div>
                        </DialogHeader>

                        <div className="space-y-4">


                            <div className="text-xs text-center text-muted-foreground leading-relaxed max-w-[280px] mx-auto">

                                Universal High-Performance <br />zero-copy data inspection engine.
                                <br />
                                <br />
                                <span className="font-mono font-black text-primary">LICENSE</span>
                            </div>
                        </div>

                        <DialogFooter className="grid sm:justify-center gap-4">
                            <Button
                                variant="outline"
                                className="w-full sm:w-auto font-bold text-[10px] uppercase tracking-widest gap-2"
                                onClick={() => openLink('https://zero.theneurocatalyst.com')}
                            >
                                <Globe className="h-3.5 w-3.5" />
                                Visit Website
                            </Button>

                            <DialogDescription className="text-xs text-center font-mono text-muted-foreground">
                                By The Neuro-Catalyst Group
                            </DialogDescription>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>

                <Button
                    variant="ghost"
                    onClick={() => openLink('https://zero.theneurocatalyst.com/docs')}
                    className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                >
                    <BookOpen className="h-4 w-4 shrink-0 group-hover:text-primary transition-colors" />
                    {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">Docs</span>}
                </Button>
            </div>

            {/* Category: Community */}
            <div className="space-y-1">
                {!isCollapsed && <p className="px-3 text-[9px] font-black uppercase tracking-widest text-muted-foreground/50 mb-2">Community</p>}

                <Button
                    variant="ghost"
                    onClick={() => openLink('https://github.com/the-neuro-catalyst/zer0')}
                    className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                >
                    <GithubIcon className="h-4 w-4 shrink-0 group-hover:text-foreground transition-colors" />
                    {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">GitHub</span>}
                </Button>

                <Button
                    variant="ghost"
                    onClick={() => openLink('https://discord.gg/HCPXuC55HV')}
                    className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                >
                    <DiscordIcon className="h-4 w-4 shrink-0 group-hover:text-[#5865F2] transition-colors" />
                    {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">Discord</span>}
                </Button>

                <Button
                    variant="ghost"
                    onClick={() => openLink('https://x.com/NeuroCatalyst')}
                    className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                >
                    <XIcon className="h-3.5 w-3.5 shrink-0 group-hover:text-foreground transition-colors" />
                    {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">Twitter / X</span>}
                </Button>

                <Button
                    variant="ghost"
                    onClick={() => openLink('https://theneurocatalyst.slack.com')}
                    className={cn("w-full justify-start gap-3 px-3 py-2 h-auto text-muted-foreground hover:bg-secondary group", isCollapsed && "justify-center")}
                >
                    <SlackIcon className="h-4 w-4 shrink-0 group-hover:text-[#E01E5A] transition-colors" />
                    {!isCollapsed && <span className="text-[10px] font-bold uppercase tracking-widest">Slack</span>}
                </Button>
            </div>
        </div>
    );
}