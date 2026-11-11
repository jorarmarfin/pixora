import { useCallback } from "react";
import { api } from "@/lib/tauri";
import { useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";

/** Importa rutas (archivos o carpetas) en el backend y las agrega a la cola. */
export function useImport() {
  const recursive = useConvertSettings((s) => s.recursive);
  const { addItems, setImporting } = useImagesStore.getState();

  return useCallback(
    async (paths: string[]) => {
      if (paths.length === 0) return 0;
      setImporting(true);
      try {
        const r = await api.importPaths(paths, recursive);
        return addItems(r.items, r.skipped);
      } finally {
        setImporting(false);
      }
    },
    [recursive, addItems, setImporting],
  );
}
