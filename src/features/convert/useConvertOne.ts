import { useCallback } from "react";
import { buildJob } from "@/lib/job";
import { api } from "@/lib/tauri";
import { useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import type { ImageItem, JobResult } from "@/types/ipc";

/** Convierte una imagen con los ajustes actuales. Los fallos quedan en el resultado. */
export function useConvertOne() {
  return useCallback(async (item: ImageItem, overwriteConfirmed = false) => {
    const { setStatus, setResult } = useImagesStore.getState();
    const job = buildJob(item, useConvertSettings.getState());
    setStatus(item.id, "processing");
    try {
      setResult(item.id, await api.processSingle(job, overwriteConfirmed));
    } catch {
      const failed: JobResult = {
        id: item.id,
        status: "failed",
        output_path: null,
        width: 0,
        height: 0,
        input_bytes: item.size_bytes,
        output_bytes: 0,
        duration_ms: 0,
        error: { code: "engine", detail: null },
      };
      setResult(item.id, failed);
    }
  }, []);
}
