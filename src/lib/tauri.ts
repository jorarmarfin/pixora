import { invoke } from "@tauri-apps/api/core";
import type { AppInfo } from "@/types/ipc";

/** Cliente tipado de comandos Tauri. */
export const api = {
  appInfo: () => invoke<AppInfo>("app_info"),
};
