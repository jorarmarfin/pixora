import { t } from "@/lib/i18n";
import { formatBytes } from "@/lib/format";
import { savingsPercent } from "@/lib/job";
import type { QueueEntry } from "@/stores/images";

/** Estado de la fila + resultado (tamaños y ahorro, o error legible). */
export function RowResult({ entry }: { entry: QueueEntry }) {
  const { status, result } = entry;
  if (status === "failed" && result?.error) {
    return (
      <span className="w-52 text-xs text-brand-accent" role="status">
        {t(`jobError.${result.error.code}`)}
      </span>
    );
  }
  if (status === "completed" && result) {
    const pct = savingsPercent(result.input_bytes, result.output_bytes);
    return (
      <span className="w-52 text-xs" role="status">
        <span className="block font-medium">{t("jobStatus.completed")}</span>
        <span className="text-muted-foreground">
          {formatBytes(result.input_bytes)} → {formatBytes(result.output_bytes)} (
          {pct >= 0 ? `${pct}% ${t("process.saved")}` : `${-pct}% ${t("process.grew")}`})
        </span>
      </span>
    );
  }
  return <span className="w-52 text-xs">{t(`jobStatus.${status}`)}</span>;
}
