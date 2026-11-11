import { create } from "zustand";

export type SectionId = "convert" | "optimize" | "resize" | "tools" | "presets" | "settings";

interface UiState {
  section: SectionId;
  setSection: (s: SectionId) => void;
}

export const useUiStore = create<UiState>((set) => ({
  section: "convert",
  setSection: (section) => set({ section }),
}));
