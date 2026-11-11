import { useEffect, useState } from "react";
import { ImagePlus } from "lucide-react";
import { t } from "@/lib/i18n";
import { listenDragDrop } from "@/lib/tauri";
import { useImagesStore } from "@/stores/images";
import { BatchBar } from "@/features/jobs/BatchBar";
import { BatchResults } from "@/features/jobs/BatchResults";
import { ImageList } from "./ImageList";
import { SkippedNotice } from "./SkippedNotice";
import { Toolbar } from "./Toolbar";
import { useImport } from "./useImport";

export function ImportView() {
  const hasItems = useImagesStore((s) => s.entries.length > 0);
  const importPaths = useImport();
  const [dragging, setDragging] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;
    listenDragDrop(
      (s) => setDragging(s === "enter"),
      (paths) => void importPaths(paths),
    )
      .then((u) => (cancelled ? u() : (unlisten = u)))
      .catch(() => undefined); // fuera de Tauri (tests / navegador)
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [importPaths]);

  return (
    <div className="relative flex h-full flex-col gap-4">
      <Toolbar />
      <SkippedNotice />
      <BatchBar />
      <BatchResults />
      <div className="min-h-0 flex-1">
        {hasItems ? (
          <ImageList />
        ) : (
          <div className="flex h-full flex-col items-center justify-center gap-3 rounded-2xl border-2 border-dashed border-border text-center">
            <ImagePlus className="size-12 text-brand-secondary" aria-hidden />
            <h2 className="text-lg font-semibold">{t("views.emptyTitle")}</h2>
            <p className="text-sm text-muted-foreground">{t("views.emptyHint")}</p>
          </div>
        )}
      </div>
      {dragging && (
        <div
          aria-hidden
          className="pointer-events-none absolute inset-0 flex items-center justify-center rounded-2xl border-2 border-brand-secondary bg-background/80 text-xl font-semibold"
        >
          {t("views.dropHere")}
        </div>
      )}
    </div>
  );
}
