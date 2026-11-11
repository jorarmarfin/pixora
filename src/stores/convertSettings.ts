import { create } from "zustand";
import type {
  ConflictPolicy,
  FitMode,
  ImageFormat,
  MetadataPolicy,
  QualityMode,
  ResizeKind,
} from "@/types/ipc";

export interface ConvertSettings {
  format: ImageFormat;
  qualityMode: QualityMode;
  quality: number;
  resizeKind: ResizeKind;
  width: number;
  height: number;
  percent: number;
  fit: FitMode;
  metadata: MetadataPolicy;
  outputDir: string;
  suffix: string;
  conflict: ConflictPolicy;
  recursive: boolean;
}

interface State extends ConvertSettings {
  patch: (p: Partial<ConvertSettings>) => void;
}

export const defaultSettings: ConvertSettings = {
  format: "webp",
  qualityMode: "lossy",
  quality: 80,
  resizeKind: "none",
  width: 1920,
  height: 1080,
  percent: 50,
  fit: "contain",
  metadata: "strip",
  outputDir: "",
  suffix: "-pixora",
  conflict: "rename",
  recursive: true,
};

export const useConvertSettings = create<State>((set) => ({
  ...defaultSettings,
  patch: (p) =>
    set((s) => {
      const next = { ...s, ...p };
      // PNG es siempre sin pérdida.
      if (next.format === "png") next.qualityMode = "lossless";
      return next;
    }),
}));
