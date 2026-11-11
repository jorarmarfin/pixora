import { create } from "zustand";
import type { ImageItem, JobResult, JobStatus, SkippedFile } from "@/types/ipc";

export interface QueueEntry {
  item: ImageItem;
  status: JobStatus;
  result?: JobResult;
}

interface ImagesState {
  entries: QueueEntry[];
  skipped: SkippedFile[];
  importing: boolean;
  setImporting: (v: boolean) => void;
  addItems: (items: ImageItem[], skipped: SkippedFile[]) => number;
  setStatus: (id: string, status: JobStatus) => void;
  setResult: (id: string, result: JobResult) => void;
  remove: (id: string) => void;
  clear: () => void;
  clearSkipped: () => void;
}

const patchEntry = (entries: QueueEntry[], id: string, p: Partial<QueueEntry>) =>
  entries.map((e) => (e.item.id === id ? { ...e, ...p } : e));

export const useImagesStore = create<ImagesState>((set, get) => ({
  entries: [],
  skipped: [],
  importing: false,
  setImporting: (importing) => set({ importing }),
  /** Agrega sin duplicar por id. Devuelve cuántas son nuevas. */
  addItems: (items, skipped) => {
    const known = new Set(get().entries.map((e) => e.item.id));
    const fresh = items.filter((i) => !known.has(i.id));
    set((s) => ({
      entries: [...s.entries, ...fresh.map((item) => ({ item, status: "queued" as const }))],
      skipped,
    }));
    return fresh.length;
  },
  setStatus: (id, status) => set((s) => ({ entries: patchEntry(s.entries, id, { status }) })),
  setResult: (id, result) =>
    set((s) => ({ entries: patchEntry(s.entries, id, { status: result.status, result }) })),
  remove: (id) => set((s) => ({ entries: s.entries.filter((e) => e.item.id !== id) })),
  clear: () => set({ entries: [], skipped: [] }),
  clearSkipped: () => set({ skipped: [] }),
}));
