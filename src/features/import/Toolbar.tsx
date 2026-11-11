import { FilePlus2, FolderPlus, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { t } from "@/lib/i18n";
import { formatBytes } from "@/lib/format";
import { pickFolder, pickImageFiles } from "@/lib/tauri";
import { useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import { useImport } from "./useImport";

export function Toolbar() {
  const importPaths = useImport();
  const { entries, importing, clear } = useImagesStore();
  const recursive = useConvertSettings((s) => s.recursive);
  const patch = useConvertSettings((s) => s.patch);
  const total = entries.reduce((n, e) => n + e.item.size_bytes, 0);

  return (
    <div className="flex flex-wrap items-center gap-3">
      <Button disabled={importing} onClick={async () => importPaths(await pickImageFiles())}>
        <FilePlus2 className="size-4" aria-hidden />
        {t("import.addFiles")}
      </Button>
      <Button
        variant="outline"
        disabled={importing}
        onClick={async () => {
          const dir = await pickFolder();
          if (dir) await importPaths([dir]);
        }}
      >
        <FolderPlus className="size-4" aria-hidden />
        {t("import.addFolder")}
      </Button>
      <label className="flex items-center gap-2 text-sm">
        <input
          type="checkbox"
          checked={recursive}
          onChange={(e) => patch({ recursive: e.target.checked })}
          className="size-4 accent-brand-primary"
        />
        {t("import.recursive")}
      </label>
      <div
        className="ml-auto flex items-center gap-3 text-sm text-muted-foreground"
        aria-live="polite"
      >
        {importing ? (
          <span>{t("import.importing")}</span>
        ) : entries.length > 0 ? (
          <span>
            {entries.length} {t("import.count")} · {formatBytes(total)} {t("import.totalSize")}
          </span>
        ) : null}
        {entries.length > 0 && (
          <Button variant="ghost" size="sm" onClick={clear}>
            <Trash2 className="size-4" aria-hidden />
            {t("import.clear")}
          </Button>
        )}
      </div>
    </div>
  );
}
