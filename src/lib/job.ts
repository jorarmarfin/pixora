import type { ConvertSettings } from "@/stores/convertSettings";
import type { ImageItem, ImageJob, ResizeOptionsDto } from "@/types/ipc";

export function buildResize(s: ConvertSettings): ResizeOptionsDto {
  switch (s.resizeKind) {
    case "width":
      return { kind: "width", px: s.width };
    case "height":
      return { kind: "height", px: s.height };
    case "percent":
      return { kind: "percent", value: s.percent };
    case "box":
      return { kind: "box", width: s.width, height: s.height, fit: s.fit };
    default:
      return { kind: "none" };
  }
}

/** Traduce los ajustes de UI al contrato `ImageJob` del backend. */
export function buildJob(item: ImageItem, s: ConvertSettings): ImageJob {
  return {
    id: item.id,
    input_path: item.path,
    output_directory: s.outputDir,
    output_format: s.format,
    quality_mode: s.format === "png" ? "lossless" : s.qualityMode,
    quality_value: s.quality,
    resize: buildResize(s),
    metadata_policy: s.metadata,
    name_suffix: s.suffix,
    conflict_policy: s.conflict,
  };
}

/** Variación de peso: positivo = ahorro. */
export function savingsPercent(input: number, output: number): number {
  if (input <= 0) return 0;
  return Math.round((1 - output / input) * 1000) / 10;
}
