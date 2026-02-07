import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export interface SystemStats {
  totalSessions: number;
  totalEvents: number;
  activeNodes: number;
  validityConfidence: number;
}

export function useSystemStats() {
  return useQuery({
    queryKey: ["system-stats"],
    queryFn: async () => {
      return await invoke<SystemStats>("get_system_stats");
    },
    refetchInterval: 30000,
  });
}
