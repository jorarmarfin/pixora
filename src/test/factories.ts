import type { ImageItem } from "@/types/ipc";

export function makeItem(i: number, over: Partial<ImageItem> = {}): ImageItem {
  return {
    id: `/fotos/img ${i}.png`,
    path: `/fotos/img ${i}.png`,
    name: `img ${i}.png`,
    width: 800,
    height: 600,
    format: "png",
    size_bytes: 1024 * (i + 1),
    ...over,
  };
}
