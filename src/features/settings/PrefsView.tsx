import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { pickFolder } from "@/lib/tauri";
import { useAppConfig } from "@/stores/appConfig";
import { useConvertSettings } from "@/stores/convertSettings";
import type { Theme } from "@/types/ipc";

const field = "h-10 rounded-lg border border-border bg-background px-3 text-sm";

export function PrefsView() {
  const { theme, concurrency, defaultOutputDir, recovered, update } = useAppConfig();
  return (
    <div className="max-w-xl space-y-6">
      <h1 className="text-xl font-semibold">{t("prefs.title")}</h1>
      {recovered && (
        <p role="alert" className="rounded-lg border border-brand-accent p-3 text-sm">
          {t("prefs.recovered")}
        </p>
      )}

      <section className="space-y-2" aria-labelledby="pref-appearance">
        <h2 id="pref-appearance" className="text-base font-semibold">
          {t("prefs.appearance")}
        </h2>
        <label htmlFor="theme" className="block text-sm font-medium">
          {t("prefs.theme")}
        </label>
        <select
          id="theme"
          className={field}
          value={theme}
          onChange={(e) => void update({ theme: e.target.value as Theme })}
        >
          <option value="dark">{t("prefs.dark")}</option>
          <option value="light">{t("prefs.light")}</option>
        </select>
      </section>

      <section className="space-y-2" aria-labelledby="pref-perf">
        <h2 id="pref-perf" className="text-base font-semibold">
          {t("prefs.performance")}
        </h2>
        <label htmlFor="concurrency" className="block text-sm font-medium">
          {t("prefs.concurrency")}: {concurrency}
        </label>
        <input
          id="concurrency"
          type="range"
          min={1}
          max={8}
          value={concurrency}
          onChange={(e) => void update({ concurrency: Number(e.target.value) })}
          className="w-full accent-brand-primary"
          aria-describedby="concurrency-hint"
        />
        <p id="concurrency-hint" className="text-xs text-muted-foreground">
          {t("prefs.concurrencyHint")}
        </p>
      </section>

      <section className="space-y-2" aria-labelledby="pref-out">
        <h2 id="pref-out" className="text-base font-semibold">
          {t("prefs.defaultOutput")}
        </h2>
        <div className="flex items-center gap-2">
          <span className="min-w-0 flex-1 truncate text-sm" title={defaultOutputDir}>
            {defaultOutputDir || t("prefs.none")}
          </span>
          <Button
            variant="outline"
            onClick={async () => {
              const d = await pickFolder();
              if (!d) return;
              await update({ defaultOutputDir: d });
              useConvertSettings.getState().patch({ outputDir: d });
            }}
          >
            {t("prefs.choose")}
          </Button>
          {defaultOutputDir && (
            <Button variant="ghost" onClick={() => void update({ defaultOutputDir: "" })}>
              {t("prefs.clear")}
            </Button>
          )}
        </div>
      </section>

      <p className="text-xs text-muted-foreground">{t("prefs.privacy")}</p>
    </div>
  );
}
