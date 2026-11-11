import { Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { useAppConfig } from "@/stores/appConfig";
import type { Preset } from "@/types/ipc";

function describe(p: Preset): string {
  const s = p.settings;
  const q =
    s.quality_mode === "lossless"
      ? t("settings.lossless")
      : `${t("settings.quality")} ${s.quality}`;
  const r =
    s.resize_kind === "none"
      ? ""
      : s.resize_kind === "width"
        ? ` · ${s.width}px`
        : s.resize_kind === "height"
          ? ` · ${s.height}px`
          : s.resize_kind === "percent"
            ? ` · ${s.percent}%`
            : ` · ${s.width}×${s.height}`;
  return `${s.format.toUpperCase()} · ${q}${r}`;
}

export function PresetsView() {
  const { builtinPresets, userPresets, applyPreset, deletePreset } = useAppConfig();
  const row = (p: Preset) => (
    <li
      key={p.id}
      className="flex items-center gap-3 rounded-xl border border-border bg-card px-4 py-3"
    >
      <div className="min-w-0 flex-1">
        <div className="truncate font-medium">
          {p.name}
          {p.builtin && (
            <span className="ml-2 rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">
              {t("presets.builtinBadge")}
            </span>
          )}
        </div>
        <div className="text-xs text-muted-foreground">{describe(p)}</div>
      </div>
      <Button size="sm" variant="outline" onClick={() => applyPreset(p.id)}>
        {t("presets.apply")}
      </Button>
      {!p.builtin && (
        <Button
          size="sm"
          variant="ghost"
          aria-label={`${t("presets.delete")} ${p.name}`}
          onClick={() => void deletePreset(p.id)}
        >
          <Trash2 className="size-4" aria-hidden />
        </Button>
      )}
    </li>
  );
  return (
    <div className="space-y-4">
      <h1 className="text-xl font-semibold">{t("presets.title")}</h1>
      <p className="text-sm text-muted-foreground">{t("presets.hint")}</p>
      <ul className="space-y-2" aria-label={t("presets.builtinGroup")}>
        {builtinPresets.map(row)}
      </ul>
      <h2 className="pt-2 text-base font-semibold">{t("presets.userGroup")}</h2>
      {userPresets.length === 0 ? (
        <p className="text-sm text-muted-foreground">{t("presets.empty")}</p>
      ) : (
        <ul className="space-y-2" aria-label={t("presets.userGroup")}>
          {userPresets.map(row)}
        </ul>
      )}
    </div>
  );
}
