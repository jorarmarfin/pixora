/** DTO compartidos con Rust (src-tauri/src/domain, commands). */
export interface AppInfo {
  name: string;
  version: string;
  engine: string;
}

export type ImageFormat = "jpeg" | "png" | "webp";

export type JobStatus =
  | "queued"
  | "validating"
  | "processing"
  | "completed"
  | "failed"
  | "cancelled"
  | "skipped";

export interface ImageItem {
  id: string;
  path: string;
  name: string;
  width: number;
  height: number;
  format: ImageFormat;
  size_bytes: number;
}

export type SkipReason = "unsupported" | "unreadable" | "corrupt";

export interface SkippedFile {
  name: string;
  reason: SkipReason;
}

export interface ImportResult {
  items: ImageItem[];
  skipped: SkippedFile[];
}

export type QualityMode = "lossy" | "lossless";
export type FitMode = "contain" | "cover" | "fit";
export type ConflictPolicy = "rename" | "skip" | "overwrite";
export type MetadataPolicy = "strip" | "keep_icc" | "keep_all";
export type ResizeKind = "none" | "width" | "height" | "percent" | "box";
