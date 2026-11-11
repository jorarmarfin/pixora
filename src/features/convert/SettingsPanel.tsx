import type { ReactNode } from "react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { pickFolder } from "@/lib/tauri";
import { useConvertSettings } from "@/stores/convertSettings";
import type { ConflictPolicy, FitMode, ImageFormat, MetadataPolicy, ResizeKind } from "@/types/ipc";

const field = "h-10 w-full rounded-lg border border-border bg-background px-3 text-sm";

function Field({
  label,
  htmlFor,
  children,
}: {
  label: string;
  htmlFor: string;
  children: ReactNode;
}) {
  return (
    <div className="flex flex-col gap-1.5">
      <label htmlFor={htmlFor} className="text-sm font-medium">
        {label}
      </label>
      {children}
    </div>
  );
}

export function SettingsPanel() {
  const s = useConvertSettings();
  const lossless = s.qualityMode === "lossless";
  const isPng = s.format === "png";
  const num = (v: string) => Math.max(1, Math.round(Number(v) || 1));

  return (
    <aside
      aria-label={t("settings.title")}
      className="flex w-80 shrink-0 flex-col gap-4 overflow-auto border-l border-border bg-card p-4"
    >
      <h2 className="text-base font-semibold">{t("settings.title")}</h2>

      <Field label={t("settings.format")} htmlFor="format">
        <select
          id="format"
          className={field}
          value={s.format}
          onChange={(e) => s.patch({ format: e.target.value as ImageFormat })}
        >
          <option value="jpeg">JPEG</option>
          <option value="png">PNG</option>
          <option value="webp">WebP</option>
        </select>
      </Field>

      <Field label={t("settings.mode")} htmlFor="mode">
        <select
          id="mode"
          className={field}
          value={s.qualityMode}
          disabled={isPng}
          onChange={(e) => s.patch({ qualityMode: e.target.value as "lossy" | "lossless" })}
        >
          <option value="lossy">{t("settings.lossy")}</option>
          <option value="lossless">{t("settings.lossless")}</option>
        </select>
        {isPng && <p className="text-xs text-muted-foreground">{t("settings.pngLossless")}</p>}
      </Field>

      <Field label={`${t("settings.quality")}: ${s.quality}`} htmlFor="quality">
        <input
          id="quality"
          type="range"
          min={1}
          max={100}
          value={s.quality}
          disabled={lossless}
          onChange={(e) => s.patch({ quality: Number(e.target.value) })}
          className="accent-brand-primary"
        />
      </Field>

      <Field label={t("settings.resize")} htmlFor="resize">
        <select
          id="resize"
          className={field}
          value={s.resizeKind}
          onChange={(e) => s.patch({ resizeKind: e.target.value as ResizeKind })}
        >
          <option value="none">{t("settings.resizeNone")}</option>
          <option value="width">{t("settings.resizeWidth")}</option>
          <option value="height">{t("settings.resizeHeight")}</option>
          <option value="percent">{t("settings.resizePercent")}</option>
          <option value="box">{t("settings.resizeBox")}</option>
        </select>
      </Field>

      {(s.resizeKind === "width" || s.resizeKind === "box") && (
        <Field label={t("settings.width")} htmlFor="width">
          <input
            id="width"
            type="number"
            min={1}
            className={field}
            value={s.width}
            onChange={(e) => s.patch({ width: num(e.target.value) })}
          />
        </Field>
      )}
      {(s.resizeKind === "height" || s.resizeKind === "box") && (
        <Field label={t("settings.height")} htmlFor="height">
          <input
            id="height"
            type="number"
            min={1}
            className={field}
            value={s.height}
            onChange={(e) => s.patch({ height: num(e.target.value) })}
          />
        </Field>
      )}
      {s.resizeKind === "percent" && (
        <Field label={t("settings.percent")} htmlFor="percent">
          <input
            id="percent"
            type="number"
            min={1}
            max={400}
            className={field}
            value={s.percent}
            onChange={(e) => s.patch({ percent: Math.min(400, num(e.target.value)) })}
          />
        </Field>
      )}
      {s.resizeKind === "box" && (
        <Field label={t("settings.fit")} htmlFor="fit">
          <select
            id="fit"
            className={field}
            value={s.fit}
            onChange={(e) => s.patch({ fit: e.target.value as FitMode })}
          >
            <option value="contain">{t("settings.fitContain")}</option>
            <option value="cover">{t("settings.fitCover")}</option>
            <option value="fit">{t("settings.fitFit")}</option>
          </select>
        </Field>
      )}

      <Field label={t("settings.metadata")} htmlFor="metadata">
        <select
          id="metadata"
          className={field}
          value={s.metadata}
          onChange={(e) => s.patch({ metadata: e.target.value as MetadataPolicy })}
        >
          <option value="strip">{t("settings.metaStrip")}</option>
          <option value="keep_all">{t("settings.metaAll")}</option>
        </select>
      </Field>

      <Field label={t("settings.outputDir")} htmlFor="outdir">
        <div className="flex items-center gap-2">
          <input
            id="outdir"
            readOnly
            className={field}
            value={s.outputDir}
            placeholder={t("settings.noDir")}
            title={s.outputDir}
          />
          <Button
            variant="outline"
            onClick={async () => {
              const d = await pickFolder();
              if (d) s.patch({ outputDir: d });
            }}
          >
            {t("settings.chooseDir")}
          </Button>
        </div>
      </Field>

      <Field label={t("settings.suffix")} htmlFor="suffix">
        <input
          id="suffix"
          className={field}
          value={s.suffix}
          onChange={(e) => s.patch({ suffix: e.target.value })}
        />
      </Field>

      <Field label={t("settings.conflict")} htmlFor="conflict">
        <select
          id="conflict"
          className={field}
          value={s.conflict}
          onChange={(e) => s.patch({ conflict: e.target.value as ConflictPolicy })}
        >
          <option value="rename">{t("settings.conflictRename")}</option>
          <option value="skip">{t("settings.conflictSkip")}</option>
          <option value="overwrite">{t("settings.conflictOverwrite")}</option>
        </select>
      </Field>
    </aside>
  );
}
