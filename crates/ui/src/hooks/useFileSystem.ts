import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener';

export function useFileSystem() {
  const openExternal = async (path: string) => {
    try {
      await openPath(path);
    } catch (err) {
      console.error("FileSystem Error: Failed to open path", err);
    }
  };

  const showInFolder = async (path: string) => {
    try {
      await revealItemInDir(path);
    } catch (err) {
      console.error("FileSystem Error: Failed to reveal item", err);
    }
  };

  return { openExternal, showInFolder };
}
