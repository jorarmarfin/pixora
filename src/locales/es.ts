export const es = {
  app: {
    name: "Pixora",
    tagline: "Convierte · Comprime · Optimiza · Procesa imágenes",
  },
  nav: {
    convert: "Convertir",
    optimize: "Comprimir",
    resize: "Redimensionar",
    tools: "Herramientas",
    presets: "Presets",
    settings: "Ajustes",
  },
  views: {
    emptyTitle: "Arrastra tus imágenes aquí",
    emptyHint: "La importación llega en la Fase 2.",
    comingSoon: "Próximamente",
  },
  status: {
    engine: "Motor",
    version: "Versión",
  },
} as const;

export type Messages = typeof es;
