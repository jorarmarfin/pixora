import { useState } from "react";
import { Play, Square } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { useBatchStore } from "@/stores/batch";
import { useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import { useBatch } from "./useBatch";

/** Acción "Convertir todo" + barra de progreso global (gradiente de marca) + cancelar. */
export function BatchBar() {
  const { start, cancel } = useBatch();
  const { running, cancelling, total, done } = useBatchStore();
  const pending = useImagesStore((s) => s.entries.filter((e) => e.status !== "completed").length);
  const outputDir = useConvertSettings((s) => s.outputDir);
  const needsConfirm = useConvertSettings((s) => s.conflict === "overwrite");
  const [armed, setArmed] = useState(false);

  if (running) {
    const pct = total > 0 ? Math.round((done / total) * 100) : 0;
    return (
      <div className="flex items-center gap-3">
        <div
          role="progressbar"
          aria-label={t("batch.progress")}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={pct}
          className="h-2.5 flex-1 overflow-hidden rounded-full bg-muted"
        >
          <div
            className="h-full rounded-full bg-gradient-to-r from-brand-primary to-brand-secondary transition-[width] duration-200"
            style={{ width: `${pct}%` }}
          />
        </div>
        <span className="w-32 text-right text-sm tabular-nums text-muted-foreground">
          {done}/{total} {t("batch.processed")}
        </span>
        <Button variant="outline" size="sm" disabled={cancelling} onClick={() => void cancel()}>
          <Square className="size-3.5" aria-hidden />
          {cancelling ? t("batch.cancelling") : t("batch.cancel")}
        </Button>
      </div>
    );
  }

  if (pending === 0) return null;
  return (
    <div className="flex items-center gap-3">
      <Button
        disabled={!outputDir}
        title={!outputDir ? t("process.needOutputDir") : undefined}
        variant={armed ? "outline" : "default"}
        onBlur={() => setArmed(false)}
        onClick={() => {
          if (needsConfirm && !armed) return setArmed(true);
          setArmed(false);
          void start(needsConfirm);
        }}
      >
        <Play className="size-4" aria-hidden />
        {armed ? t("batch.confirmOverwrite") : `${t("batch.convertAll")} (${pending})`}
      </Button>
      {!outputDir && (
        <span className="text-xs text-muted-foreground">{t("process.needOutputDir")}</span>
      )}
    </div>
  );
}
