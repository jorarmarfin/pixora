import { Construction } from "lucide-react";
import { t } from "@/lib/i18n";

/** Marcador: las herramientas extra aún no existen. */
export function ToolsView() {
  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 text-center">
      <Construction className="size-12 text-brand-secondary" aria-hidden />
      <h1 className="text-xl font-semibold">{t("tools.title")}</h1>
      <p className="max-w-sm text-sm text-muted-foreground">{t("tools.hint")}</p>
    </div>
  );
}
