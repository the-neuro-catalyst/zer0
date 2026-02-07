import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "@/components/ui/sonner";
import DataInspector from "@/pages/DataInspector";
import HistoryPage from "@/pages/History";
import VaultPage from "@/pages/Vault";
import SettingsPage from "@/pages/Settings";
import { Sidebar } from "@/components/Sidebar";
import { WindowManager } from "@/components/WindowManager";
import { ThemeProvider } from "@/components/ThemeProvider";

const queryClient = new QueryClient();

function AppLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="h-screen bg-background font-sans text-foreground flex flex-col overflow-hidden border border-border/20 rounded-lg shadow-2xl">
      <WindowManager />
      <div className="flex-1 flex overflow-hidden">
        <Sidebar />
        <main className="flex-1 flex flex-col overflow-y-auto relative no-scrollbar">
          <div className="flex-1">
            {children}
          </div>
        </main>
      </div>
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider attribute="class" defaultTheme="dark" enableSystem>

        <BrowserRouter>
          <AppLayout>
            <Routes>
              <Route path="/" element={<DataInspector />} />
              <Route path="/history" element={<HistoryPage />} />
              <Route path="/vault" element={<VaultPage />} />
              <Route path="/settings" element={<SettingsPage />} />
              <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
          </AppLayout>
          <Toaster position="bottom-right" theme="dark" />
        </BrowserRouter>
      </ThemeProvider>

    </QueryClientProvider>
  );
}

export default App;

