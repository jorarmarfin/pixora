import type { ConvertSettings } from "@/stores/convertSettings";
import type { SectionId } from "@/stores/ui";
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

/** Comprimir y Redimensionar conservan el formato de cada imagen; Convertir usa el elegido. */
export const keepsFormat = (mode: SectionId) => mode === "optimize" || mode === "resize";

/** Motivo por el que la sección no puede ejecutarse aún (clave i18n), o null. */
export function modeIssue(mode: SectionId, s: ConvertSettings): "process.needResize" | null {
  return mode === "resize" && s.resizeKind === "none" ? "process.needResize" : null;
}

/** Traduce los ajustes de UI al contrato `ImageJob` del backend. */
export function buildJob(
  item: ImageItem,
  s: ConvertSettings,
  mode: SectionId = "convert",
): ImageJob {
  const format = keepsFormat(mode) ? item.format : s.format;
  return {
    id: item.id,
    input_path: item.path,
    output_directory: s.outputDir,
    output_format: format,
    quality_mode: format === "png" ? "lossless" : s.qualityMode,
    quality_value: s.quality,
    resize: mode === "optimize" ? { kind: "none" } : buildResize(s),
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
