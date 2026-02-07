export function Footer() {
  return (
    <footer className="p-8 pb-12 space-y-6 border-t border-border/10 bg-background/50 backdrop-blur-sm shrink-0">
      <div className="flex flex-col items-center text-center space-y-4">
        <div className="h-12 w-12 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center">
          <span className="text-2xl font-black text-primary">Z</span>
        </div>
        <div>
          <p className="text-[10px] font-black tracking-[0.3em] uppercase opacity-80 text-foreground">ZERO</p>
          <p className="text-[8px] font-bold text-muted-foreground uppercase tracking-[0.2em] mt-1 opacity-40">Build v1.0.0-STABLE [Linux x64]</p>
        </div>
      </div>
    </footer>
  );
}
