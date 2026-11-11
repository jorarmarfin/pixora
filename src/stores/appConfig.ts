import { create } from "zustand";
import { api } from "@/lib/tauri";
import type { AppConfig, Preset, PresetSettings, Theme } from "@/types/ipc";
import { useConvertSettings, type ConvertSettings } from "./convertSettings";

export function applyTheme(theme: Theme) {
  const root = document.documentElement;
  root.classList.toggle("dark", theme === "dark");
  root.style.colorScheme = theme;
}

/** Ajustes de UI → preset (y viceversa). */
export function toPresetSettings(s: ConvertSettings): PresetSettings {
  return {
    format: s.format,
    quality_mode: s.qualityMode,
    quality: s.quality,
    resize_kind: s.resizeKind,
    width: s.width,
    height: s.height,
    percent: s.percent,
    fit: s.fit,
    metadata: s.metadata,
  };
}

export function fromPresetSettings(p: PresetSettings): Partial<ConvertSettings> {
  return {
    format: p.format,
    qualityMode: p.quality_mode,
    quality: p.quality,
    resizeKind: p.resize_kind,
    width: p.width,
    height: p.height,
    percent: p.percent,
    fit: p.fit,
    metadata: p.metadata,
  };
}

interface State {
  loaded: boolean;
  recovered: boolean;
  theme: Theme;
  concurrency: number;
  defaultOutputDir: string;
  lastPresetId: string | null;
  userPresets: Preset[];
  builtinPresets: Preset[];
  load: () => Promise<void>;
  /** Cambia preferencias y las guarda en disco. */
  update: (p: Partial<Pick<State, "theme" | "concurrency" | "defaultOutputDir">>) => Promise<void>;
  applyPreset: (id: string) => void;
  savePreset: (name: string, settings: PresetSettings) => Promise<void>;
  deletePreset: (id: string) => Promise<void>;
}

const snapshot = (s: State): AppConfig => ({
  version: 1,
  theme: s.theme,
  concurrency: s.concurrency,
  default_output_dir: s.defaultOutputDir,
  last_preset_id: s.lastPresetId,
  presets: s.userPresets,
});

export const useAppConfig = create<State>((set, get) => {
  const persist = async () => {
    try {
      await api.saveConfig(snapshot(get()));
    } catch {
      /* la UI sigue funcionando con los valores en memoria */
    }
  };
  return {
    loaded: false,
    recovered: false,
    theme: "dark",
    concurrency: 3,
    defaultOutputDir: "",
    lastPresetId: null,
    userPresets: [],
    builtinPresets: [],

    load: async () => {
      try {
        const c = await api.loadConfig();
        set({
          loaded: true,
          recovered: c.recovered,
          theme: c.theme,
          concurrency: c.concurrency,
          defaultOutputDir: c.default_output_dir,
          lastPresetId: c.last_preset_id,
          userPresets: c.presets,
          builtinPresets: c.builtin_presets,
        });
        applyTheme(c.theme);
        const ui = useConvertSettings.getState();
        if (!ui.outputDir && c.default_output_dir) ui.patch({ outputDir: c.default_output_dir });
        const last = [...c.builtin_presets, ...c.presets].find((p) => p.id === c.last_preset_id);
        if (last) ui.patch(fromPresetSettings(last.settings));
      } catch {
        set({ loaded: true }); // fuera de Tauri o sin acceso: valores por defecto
        applyTheme(get().theme);
      }
    },

    update: async (p) => {
      set(p);
      if (p.theme) applyTheme(p.theme);
      await persist();
    },

    applyPreset: (id) => {
      const preset = [...get().builtinPresets, ...get().userPresets].find((x) => x.id === id);
      if (!preset) return;
      useConvertSettings.getState().patch(fromPresetSettings(preset.settings));
      set({ lastPresetId: id });
      void persist();
    },

    savePreset: async (name, settings) => {
      const trimmed = name.trim();
      if (!trimmed) return;
      const preset: Preset = {
        id: crypto.randomUUID(),
        name: trimmed,
        builtin: false,
        settings,
      };
      set((s) => ({ userPresets: [...s.userPresets, preset], lastPresetId: preset.id }));
      await persist();
    },

    deletePreset: async (id) => {
      set((s) => ({
        userPresets: s.userPresets.filter((p) => p.id !== id),
        lastPresetId: s.lastPresetId === id ? null : s.lastPresetId,
      }));
      await persist();
    },
  };
});
