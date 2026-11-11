import { useCallback } from "react";
import { buildJob } from "@/lib/job";
import { api, onBatchProgress } from "@/lib/tauri";
import { useBatchStore } from "@/stores/batch";
import { useAppConfig } from "@/stores/appConfig";
import { useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import type { BatchProgress, JobResult } from "@/types/ipc";

function applyProgress(p: BatchProgress) {
  const { setStatus, setResult } = useImagesStore.getState();
  p.started.forEach((id) => setStatus(id, "processing"));
  p.finished.forEach((r) => setResult(r.id, r));
  useBatchStore.getState().progress(p.done);
}

/** Convierte todas las imágenes pendientes (en cola o con error) con los ajustes actuales. */
export function useBatch() {
  const start = useCallback(async (overwriteConfirmed = false) => {
    const batch = useBatchStore.getState();
    if (batch.running) return;
    const settings = useConvertSettings.getState();
    const { entries, setStatus, setResult } = useImagesStore.getState();
    const pending = entries.filter((e) => e.status !== "completed").map((e) => e.item);
    if (pending.length === 0 || !settings.outputDir) return;

    batch.begin(pending.length, settings.outputDir);
    pending.forEach((i) => setStatus(i.id, "queued"));
    let unlisten: (() => void) | undefined;
    try {
      unlisten = await onBatchProgress(applyProgress);
      const report = await api.startBatch(
        pending.map((i) => buildJob(i, settings)),
        overwriteConfirmed,
        useAppConfig.getState().concurrency,
      );
      // El resultado final es la fuente de verdad (el evento puede haberse agrupado).
      report.results.forEach((r: JobResult) => setResult(r.id, r));
      useBatchStore.getState().finish(report.summary);
    } catch {
      pending.forEach((i) => setStatus(i.id, "failed"));
      useBatchStore.getState().abort();
    } finally {
      unlisten?.();
    }
  }, []);

  const cancel = useCallback(async () => {
    useBatchStore.getState().setCancelling();
    await api.cancelBatch();
  }, []);

  return { start, cancel };
}
