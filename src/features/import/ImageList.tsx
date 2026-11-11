import { useRef } from "react";
import { X } from "lucide-react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { t } from "@/lib/i18n";
import { formatBytes } from "@/lib/format";
import { useImagesStore } from "@/stores/images";
import { Thumbnail } from "./Thumbnail";

const ROW_HEIGHT = 64;

export function ImageList() {
  const entries = useImagesStore((s) => s.entries);
  const remove = useImagesStore((s) => s.remove);
  const parentRef = useRef<HTMLDivElement>(null);

  // eslint-disable-next-line react-hooks/incompatible-library -- API de TanStack Virtual
  const virtualizer = useVirtualizer({
    count: entries.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => ROW_HEIGHT,
    overscan: 6,
    initialRect: { width: 800, height: 600 },
  });

  return (
    <div
      ref={parentRef}
      role="list"
      aria-label={t("import.list")}
      className="h-full overflow-auto rounded-2xl border border-border bg-card"
    >
      <div style={{ height: virtualizer.getTotalSize(), position: "relative" }}>
        {virtualizer.getVirtualItems().map((v) => {
          const { item, status } = entries[v.index];
          return (
            <div
              key={item.id}
              role="listitem"
              className="absolute inset-x-0 flex items-center gap-3 border-b border-border px-3"
              style={{ height: v.size, transform: `translateY(${v.start}px)` }}
            >
              <Thumbnail path={item.path} />
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm font-medium" title={item.path}>
                  {item.name}
                </p>
                <p className="truncate text-xs text-muted-foreground">{item.path}</p>
              </div>
              <span className="w-28 text-xs text-muted-foreground">
                {item.width} × {item.height}
              </span>
              <span className="w-14 text-xs uppercase text-muted-foreground">{item.format}</span>
              <span className="w-20 text-right text-xs text-muted-foreground">
                {formatBytes(item.size_bytes)}
              </span>
              <span className="w-24 text-xs">{t(`jobStatus.${status}`)}</span>
              <button
                type="button"
                onClick={() => remove(item.id)}
                aria-label={`${t("import.remove")} ${item.name}`}
                className="rounded-md p-1.5 text-muted-foreground hover:bg-muted hover:text-foreground"
              >
                <X className="size-4" aria-hidden />
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
}
