import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppInfo, ImageJob, ImportResult, JobResult } from "@/types/ipc";

/** Cliente tipado de comandos Tauri. */
export const api = {
  appInfo: () => invoke<AppInfo>("app_info"),
  importPaths: (paths: string[], recursive: boolean) =>
    invoke<ImportResult>("import_paths", { paths, recursive }),
  processSingle: (job: ImageJob, overwriteConfirmed: boolean) =>
    invoke<JobResult>("process_single", {
      req: { job, overwrite_confirmed: overwriteConfirmed },
    }),
  getThumbnail: (path: string) => invoke<string>("get_thumbnail", { path }),
};

export async function pickImageFiles(): Promise<string[]> {
  const r = await open({
    multiple: true,
    filters: [{ name: "Imágenes", extensions: ["jpg", "jpeg", "png", "webp"] }],
  });
  return r ?? [];
}

export async function pickFolder(): Promise<string | null> {
  const r = await open({ directory: true, multiple: false });
  return r ?? null;
}

export type DragState = "enter" | "leave" | "drop";

/** Escucha drag & drop nativo (rutas reales). Devuelve función para desuscribir. */
export async function listenDragDrop(
  onState: (s: DragState) => void,
  onDrop: (paths: string[]) => void,
): Promise<() => void> {
  return getCurrentWebview().onDragDropEvent((e) => {
    const p = e.payload;
    if (p.type === "enter") onState("enter");
    else if (p.type === "leave") onState("leave");
    else if (p.type === "drop") {
      onState("drop");
      onDrop(p.paths);
    }
  });
}
