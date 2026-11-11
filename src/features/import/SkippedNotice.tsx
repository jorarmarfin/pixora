import { AlertTriangle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { useImagesStore } from "@/stores/images";

export function SkippedNotice() {
  const skipped = useImagesStore((s) => s.skipped);
  const clearSkipped = useImagesStore((s) => s.clearSkipped);
  if (skipped.length === 0) return null;
  return (
    <div
      role="alert"
      className="rounded-xl border border-brand-accent/50 bg-brand-accent/10 p-3 text-sm"
    >
      <div className="flex items-center gap-2 font-medium">
        <AlertTriangle className="size-4 text-brand-accent" aria-hidden />
        {t("import.skippedTitle")} ({skipped.length})
        <Button variant="ghost" size="sm" className="ml-auto" onClick={clearSkipped}>
          {t("import.dismiss")}
        </Button>
      </div>
      <ul className="mt-2 max-h-24 list-disc overflow-auto pl-6 text-muted-foreground">
        {skipped.map((s, i) => (
          <li key={`${s.name}-${i}`}>
            {s.name} — {t(`skip.${s.reason}`)}
          </li>
        ))}
      </ul>
    </div>
  );
}
