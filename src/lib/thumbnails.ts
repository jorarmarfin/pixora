import { api } from "@/lib/tauri";

const MAX_CONCURRENT = 3;
const cache = new Map<string, Promise<string>>();
const waiting: (() => void)[] = [];
let active = 0;

function acquire(): Promise<void> {
  if (active < MAX_CONCURRENT) {
    active++;
    return Promise.resolve();
  }
  return new Promise((res) => waiting.push(res));
}

function release() {
  const next = waiting.shift();
  if (next) next();
  else active--;
}

/** Miniatura con caché y concurrencia acotada (evita saturar el backend). */
export function loadThumbnail(path: string): Promise<string> {
  let p = cache.get(path);
  if (!p) {
    p = acquire()
      .then(() => api.getThumbnail(path))
      .finally(release);
    p.catch(() => cache.delete(path));
    cache.set(path, p);
  }
  return p;
}

export function clearThumbnailCache() {
  cache.clear();
}
