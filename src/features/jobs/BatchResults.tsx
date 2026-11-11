import { useMemo } from "react";
import { FolderOpen, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { formatBytes } from "@/lib/format";
import { t } from "@/lib/i18n";
import { savingsPercent } from "@/lib/job";
import { api } from "@/lib/tauri";
import { useBatchStore } from "@/stores/batch";
import { useImagesStore } from "@/stores/images";

function Stat({ label, value, accent }: { label: string; value: number; accent?: boolean }) {
  return (
    <div className="rounded-xl bg-muted px-3 py-2">
      <div className={`text-xl font-semibold tabular-nums ${accent ? "text-brand-accent" : ""}`}>
        {value}
      </div>
      <div className="text-xs text-muted-foreground">{label}</div>
    </div>
  );
}

/** Resumen del último lote: conteos, ahorro absoluto/% y acceso a la carpeta de destino. */
export function BatchResults() {
  const { summary, outputDir, dismiss } = useBatchStore();
  const entries = useImagesStore((s) => s.entries);
  const failures = useMemo(() => entries.filter((e) => e.status === "failed"), [entries]);
  if (!summary) return null;
  const pct = savingsPercent(summary.input_bytes, summary.output_bytes);

  return (
    <section
      aria-label={t("batch.resultsTitle")}
      className="space-y-3 rounded-2xl border border-border bg-card p-4"
    >
      <div className="flex items-center gap-3">
        <h2 className="text-base font-semibold">
          {summary.was_cancelled ? t("batch.cancelledTitle") : t("batch.resultsTitle")}
        </h2>
        <div className="ml-auto flex gap-2">
          <Button
            size="sm"
            variant="outline"
            disabled={!outputDir}
            onClick={() => void api.revealFolder(outputDir)}
          >
            <FolderOpen className="size-4" aria-hidden />
            {t("batch.openFolder")}
          </Button>
          <Button size="sm" variant="ghost" aria-label={t("batch.dismiss")} onClick={dismiss}>
            <X className="size-4" aria-hidden />
          </Button>
        </div>
      </div>
      <div className="grid grid-cols-2 gap-2 sm:grid-cols-4">
        <Stat label={t("batch.ok")} value={summary.completed} />
        <Stat label={t("batch.failed")} value={summary.failed} accent={summary.failed > 0} />
        <Stat label={t("batch.skipped")} value={summary.skipped} />
        <Stat label={t("batch.cancelled")} value={summary.cancelled} />
      </div>
      <p className="text-sm" role="status">
        <span className="text-muted-foreground">{t("batch.saving")}: </span>
        <span className="font-medium">
          {summary.saved_bytes >= 0
            ? formatBytes(summary.saved_bytes)
            : `-${formatBytes(-summary.saved_bytes)}`}{" "}
          ({pct}%)
        </span>
        <span className="text-muted-foreground">
          {" "}
          · {formatBytes(summary.input_bytes)} → {formatBytes(summary.output_bytes)}
        </span>
      </p>
      {failures.length > 0 && (
        <details className="text-sm">
          <summary className="cursor-pointer text-brand-accent">
            {t("batch.errors")} ({failures.length})
          </summary>
          <ul className="mt-2 max-h-32 space-y-1 overflow-auto text-xs">
            {failures.map((e) => (
              <li key={e.item.id}>
                <span className="font-medium">{e.item.name}</span> —{" "}
                {e.result?.error ? t(`jobError.${e.result.error.code}`) : ""}
              </li>
            ))}
          </ul>
        </details>
      )}
    </section>
  );
}
