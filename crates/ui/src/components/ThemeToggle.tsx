import { Moon, Sun, Monitor } from "lucide-react";
import { useTheme } from "next-themes";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";

export function ThemeToggle() {
    const { theme, setTheme } = useTheme();
    return (
        <Tabs value={theme} onValueChange={setTheme} className="w-fit">
            <TabsList className="bg-secondary/10 border border-border/40 p-1 h-9">
                <TabsTrigger value="light" className="gap-2 text-[10px] font-bold uppercase tracking-widest px-3 h-7 data-[state=active]:bg-background data-[state=active]:text-primary data-[state=active]:shadow-none">
                    <Sun className="h-3 w-3" /> Light
                </TabsTrigger>
                <TabsTrigger value="dark" className="gap-2 text-[10px] font-bold uppercase tracking-widest px-3 h-7 data-[state=active]:bg-background data-[state=active]:text-primary data-[state=active]:shadow-none">
                    <Moon className="h-3 w-3" /> Dark
                </TabsTrigger>
                <TabsTrigger value="system" className="gap-2 text-[10px] font-bold uppercase tracking-widest px-3 h-7 data-[state=active]:bg-background data-[state=active]:text-primary data-[state=active]:shadow-none">
                    <Monitor className="h-3 w-3" /> System
                </TabsTrigger>
            </TabsList>
        </Tabs>
    );
}
