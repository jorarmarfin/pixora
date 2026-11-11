import "@testing-library/jest-dom/vitest";
import { vi } from "vitest";

class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}
globalThis.ResizeObserver ??= ResizeObserverStub as unknown as typeof ResizeObserver;

// Fuera de Tauri no existe el puente IPC: se simula todo el cliente.
vi.mock("@/lib/tauri", () => ({
  api: {
    appInfo: vi.fn(),
    importPaths: vi.fn(),
    processSingle: vi.fn(),
    startBatch: vi.fn(),
    loadConfig: vi.fn(),
    saveConfig: vi.fn().mockImplementation(async (c) => c),
    cancelBatch: vi.fn().mockResolvedValue(true),
    revealFolder: vi.fn().mockResolvedValue(undefined),
    getThumbnail: vi.fn().mockResolvedValue("data:image/jpeg;base64,AAAA"),
  },
  pickImageFiles: vi.fn().mockResolvedValue([]),
  pickFolder: vi.fn().mockResolvedValue(null),
  onBatchProgress: vi.fn().mockResolvedValue(() => {}),
  listenDragDrop: vi.fn().mockResolvedValue(() => {}),
}));

// jsdom no hace layout: el virtualizador necesitaría un viewport con tamaño.
Object.defineProperty(HTMLElement.prototype, "clientHeight", { configurable: true, value: 600 });
Object.defineProperty(HTMLElement.prototype, "clientWidth", { configurable: true, value: 800 });
Element.prototype.getBoundingClientRect = () =>
  ({
    width: 800,
    height: 600,
    top: 0,
    left: 0,
    right: 800,
    bottom: 600,
    x: 0,
    y: 0,
    toJSON() {},
  }) as DOMRect;
