# Changelog

## [Sin publicar] — MVP
### Añadido
- Importación de archivos y carpetas (drag & drop), lista virtualizada con miniaturas y metadatos.
- Motor libvips: JPEG/PNG/WebP, calidad, redimensionado, EXIF, alfa, metadatos.
- Escritura segura (temporal + publicación atómica), política de conflictos.
- Lotes concurrentes con progreso, cancelación cooperativa y resumen de ahorro.
- Presets (integrados y propios), preferencias persistentes, tema claro/oscuro.
- CI de verificación (lint, tipos, tests, clippy).
### Seguridad
- Límite de píxeles en miniaturas; CSP restrictivo.
