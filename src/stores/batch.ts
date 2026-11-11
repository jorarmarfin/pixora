import { create } from "zustand";
import type { BatchSummary } from "@/types/ipc";

interface BatchState {
  running: boolean;
  cancelling: boolean;
  total: number;
  done: number;
  summary: BatchSummary | null;
  /** Carpeta de salida usada por el último lote (para "abrir carpeta"). */
  outputDir: string;
  begin: (total: number, outputDir: string) => void;
  progress: (done: number) => void;
  setCancelling: () => void;
  finish: (summary: BatchSummary) => void;
  abort: () => void;
  dismiss: () => void;
}

export const useBatchStore = create<BatchState>((set) => ({
  running: false,
  cancelling: false,
  total: 0,
  done: 0,
  summary: null,
  outputDir: "",
  begin: (total, outputDir) =>
    set({ running: true, cancelling: false, total, done: 0, summary: null, outputDir }),
  progress: (done) => set((s) => ({ done: Math.max(s.done, done) })),
  setCancelling: () => set({ cancelling: true }),
  finish: (summary) => set({ running: false, cancelling: false, summary, done: summary.total }),
  abort: () => set({ running: false, cancelling: false }),
  dismiss: () => set({ summary: null }),
}));
