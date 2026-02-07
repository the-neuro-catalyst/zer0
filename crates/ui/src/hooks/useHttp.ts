import { useState } from 'react';

export function useHttp() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const request = async (url: string, options: RequestInit = {}) => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await fetch(url, {
        method: 'GET',
        ...options,
      });
      if (!response.ok) {
        throw new Error(`HTTP Error: ${response.status} ${response.statusText}`);
      }
      return response;
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : "Unknown Network Error";
      setError(msg);
      throw new Error(msg);
    } finally {
      setIsLoading(false);
    }
  };

  return { request, isLoading, error };
}
