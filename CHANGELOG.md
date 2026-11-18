# Changelog

## [1.0.0] — 2026-11-18
### Añadido
- Importación de archivos y carpetas (drag & drop), lista virtualizada con miniaturas y metadatos.
- Motor libvips: JPEG/PNG/WebP, calidad, redimensionado, EXIF, alfa, metadatos.
- Escritura segura (temporal + publicación atómica), política de conflictos.
- Lotes concurrentes con progreso, cancelación cooperativa y resumen de ahorro.
- Presets (integrados y propios), preferencias persistentes, tema claro/oscuro.
- CI de verificación (lint, tipos, tests, clippy).
- Instalador de Windows (Inno Setup), paquete `.deb` de Linux y paquete MSIX para Microsoft Store (PixoraL).
- Scripts de compilación `build-linux.sh` y `build-windows.ps1`.
### Seguridad
- Límite de píxeles en miniaturas; CSP restrictivo.
