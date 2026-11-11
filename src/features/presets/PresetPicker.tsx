import { useState } from "react";
import { Save } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { toPresetSettings, useAppConfig } from "@/stores/appConfig";
import { useConvertSettings } from "@/stores/convertSettings";
import type { PresetSettings } from "@/types/ipc";

const field = "h-10 w-full rounded-lg border border-border bg-background px-3 text-sm";

const same = (a: PresetSettings, b: PresetSettings) =>
  (Object.keys(a) as (keyof PresetSettings)[]).every((k) => a[k] === b[k]);

/** Selector de presets: aplica uno, detecta "Personalizado" y guarda los ajustes actuales. */
export function PresetPicker() {
  const { builtinPresets, userPresets, applyPreset, savePreset } = useAppConfig();
  const ui = useConvertSettings();
  const [naming, setNaming] = useState(false);
  const [name, setName] = useState("");
  const current = toPresetSettings(ui);
  const match = [...builtinPresets, ...userPresets].find((p) => same(p.settings, current));

  return (
    <div className="flex flex-col gap-1.5">
      <label htmlFor="preset" className="text-sm font-medium">
        {t("presets.label")}
      </label>
      <select
        id="preset"
        className={field}
        value={match?.id ?? "custom"}
        onChange={(e) => e.target.value !== "custom" && applyPreset(e.target.value)}
      >
        <option value="custom" disabled={!!match}>
          {t("presets.custom")}
        </option>
        <optgroup label={t("presets.builtinGroup")}>
          {builtinPresets.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </optgroup>
        {userPresets.length > 0 && (
          <optgroup label={t("presets.userGroup")}>
            {userPresets.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </optgroup>
        )}
      </select>
      {naming ? (
        <form
          className="flex gap-2"
          onSubmit={(e) => {
            e.preventDefault();
            if (!name.trim()) return;
            void savePreset(name, current);
            setName("");
            setNaming(false);
          }}
        >
          <input
            autoFocus
            aria-label={t("presets.namePlaceholder")}
            placeholder={t("presets.namePlaceholder")}
            maxLength={60}
            className={field}
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => e.key === "Escape" && setNaming(false)}
          />
          <Button type="submit" size="sm" disabled={!name.trim()}>
            {t("presets.save")}
          </Button>
        </form>
      ) : (
        !match && (
          <Button variant="ghost" size="sm" className="self-start" onClick={() => setNaming(true)}>
            <Save className="size-4" aria-hidden />
            {t("presets.saveAs")}
          </Button>
        )
      )}
    </div>
  );
}
