import { useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import {
  Search,
  History,
  Lock,
  Settings,
  ChevronLeft,
  ChevronRight,
  LucideIcon
} from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { SidebarActions } from "@/components/SidebarActions";

interface NavItemProps {
  icon: LucideIcon;
  label: string;
  path: string;
  isCollapsed: boolean;
  isActive: boolean;
  onClick: (path: string) => void;
}

function NavItem({ icon: Icon, label, path, isCollapsed, isActive, onClick }: NavItemProps) {
  return (
    <Button
      variant={isActive ? "default" : "ghost"}
      onClick={() => onClick(path)}
      className={cn(
        "w-full justify-start gap-3 px-3 py-2.5 h-auto rounded-xl transition-all duration-200 group relative",
        isActive
          ? "shadow-lg shadow-primary/20"
          : "text-muted-foreground hover:bg-secondary hover:text-foreground"
      )}
    >
      <Icon className={cn("h-5 w-5 shrink-0")} />
      {!isCollapsed && (
        <span className="text-xs font-bold uppercase tracking-widest overflow-hidden whitespace-nowrap">
          {label}
        </span>
      )}
      {isCollapsed && (
        <div className="absolute left-14 px-3 py-1.5 bg-popover text-popover-foreground text-[10px] font-bold uppercase tracking-widest rounded-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none shadow-xl border border-border z-50">
          {label}
        </div>
      )}
    </Button>
  );
}

export function Sidebar() {
  const [isCollapsed, setIsCollapsed] = useState(true);
  const navigate = useNavigate();
  const location = useLocation();

  const navItems = [
    { icon: Search, label: "Inspector", path: "/" },
    { icon: History, label: "History", path: "/history" },
    { icon: Lock, label: "Vault", path: "/vault" },
    { icon: Settings, label: "Settings", path: "/settings" },
  ];

  return (
    <div
      className={cn(
        "h-full bg-background border-r border-border/40 flex flex-col transition-all duration-300 relative z-40",
        isCollapsed ? "w-16" : "w-56"
      )}
    >
      <nav className="flex-1 px-2.5 pt-4 space-y-2">
        {navItems.map((item) => (
          <NavItem
            key={item.path}
            {...item}
            isCollapsed={isCollapsed}
            isActive={location.pathname === item.path}
            onClick={navigate}
          />
        ))}
      </nav>

      <SidebarActions isCollapsed={isCollapsed} />

      <div className="p-2.5 border-t border-border/10">
        <Button variant="ghost" onClick={() => setIsCollapsed(!isCollapsed)} className="w-full h-10 hover:bg-secondary text-muted-foreground">
          {isCollapsed ? <ChevronRight className="h-4 w-4" /> : <ChevronLeft className="h-4 w-4" />}
        </Button>
      </div>
    </div>
  );
}