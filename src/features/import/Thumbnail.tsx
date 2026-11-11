import { useEffect, useState } from "react";
import { ImageOff } from "lucide-react";
import { loadThumbnail } from "@/lib/thumbnails";

export function Thumbnail({ path }: { path: string }) {
  const [src, setSrc] = useState<string | null>(null);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let alive = true;
    loadThumbnail(path)
      .then((s) => alive && setSrc(s))
      .catch(() => alive && setFailed(true));
    return () => {
      alive = false;
    };
  }, [path]);

  return (
    <div className="flex size-12 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-muted">
      {src ? (
        <img src={src} alt="" className="size-full object-cover" />
      ) : failed ? (
        <ImageOff className="size-5 text-muted-foreground" aria-hidden />
      ) : null}
    </div>
  );
}
