# Matriz de formatos (MVP)

| Entrada \ Salida | JPEG | PNG | WebP |
|---|---|---|---|
| JPEG | ✓ recomprime | ✓ | ✓ |
| PNG | ✓ (alfa se aplana sobre blanco) | ✓ (alfa conservado) | ✓ (alfa conservado) |
| WebP | ✓ (alfa aplanado) | ✓ | ✓ |

- Entrada: `.jpg .jpeg .png .webp` (por extensión; la cabecera se valida al importar).
- Límite: 250 MP por imagen (procesado) y 64 MP para generar miniatura.
- CMYK se convierte a sRGB; con *quitar metadatos* el perfil ICC se transforma a sRGB antes de descartarlo.
- Fuera del MVP: GIF, AVIF, HEIC, TIFF, SVG.
