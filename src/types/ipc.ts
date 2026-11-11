/** DTO compartidos con Rust (src-tauri/src/domain, commands). */
export interface AppInfo {
  name: string;
  version: string;
  engine: string;
}

export type ImageFormat = "jpeg" | "png" | "webp";

export type JobStatus =
  "queued" | "validating" | "processing" | "completed" | "failed" | "cancelled" | "skipped";

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

export type ResizeOptionsDto =
  | { kind: "none" }
  | { kind: "width"; px: number }
  | { kind: "height"; px: number }
  | { kind: "percent"; value: number }
  | { kind: "box"; width: number; height: number; fit: FitMode };

export interface ImageJob {
  id: string;
  input_path: string;
  output_directory: string;
  output_format: ImageFormat;
  quality_mode: QualityMode;
  quality_value: number;
  resize: ResizeOptionsDto;
  metadata_policy: MetadataPolicy;
  name_suffix: string;
  conflict_policy: ConflictPolicy;
}

export type JobErrorCode =
  | "input_not_found"
  | "unreadable"
  | "corrupt"
  | "unsupported"
  | "too_large"
  | "invalid_output_dir"
  | "permission_denied"
  | "disk_full"
  | "source_equals_destination"
  | "overwrite_not_confirmed"
  | "invalid_options"
  | "engine"
  | "io";

export interface JobError {
  code: JobErrorCode;
  detail: string | null;
}

export interface JobResult {
  id: string;
  status: JobStatus;
  output_path: string | null;
  width: number;
  height: number;
  input_bytes: number;
  output_bytes: number;
  duration_ms: number;
  error: JobError | null;
}

export interface BatchSummary {
  total: number;
  completed: number;
  failed: number;
  skipped: number;
  cancelled: number;
  input_bytes: number;
  output_bytes: number;
  /** Negativo si la salida pesa más que la entrada. */
  saved_bytes: number;
  duration_ms: number;
  was_cancelled: boolean;
}

export interface BatchReport {
  summary: BatchSummary;
  results: JobResult[];
}

/** Evento `batch://progress`: cambios acumulados (throttled en Rust). */
export interface BatchProgress {
  total: number;
  done: number;
  started: string[];
  finished: JobResult[];
}
