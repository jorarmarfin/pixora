import { ImagePlus } from "lucide-react";
import { t } from "@/lib/i18n";

export function EmptyState() {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 rounded-2xl border-2 border-dashed border-border text-center">
      <ImagePlus className="size-12 text-brand-secondary" aria-hidden />
      <h2 className="text-lg font-semibold">{t("views.emptyTitle")}</h2>
      <p className="text-sm text-muted-foreground">{t("views.emptyHint")}</p>
    </div>
  );
}
