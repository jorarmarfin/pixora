# Checklist de criterios MVP (spec §11)

Leyenda: ✅ verificado con tests · 🟡 implementado, falta prueba manual en la app · ⏳ Fase 7

- ✅ Convertir formatos garantizados con parámetros válidos (matriz de regresión)
- ✅ Los originales quedan intactos por defecto
- ✅ Errores por archivo sin abortar el lote (lote de 100 con errores)
- ✅ Cancelar deja salidas consistentes, sin `.tmp`
- ✅ Resultados con bytes y ahorro (`BatchSummary`)
- ✅ Pruebas: corruptos, alfa, EXIF, Unicode/espacios, conflictos, permisos
- ✅ Presets y preferencias persisten (tests de persistencia y corrupción)
- 🟡 `./run.sh` arranca en Ubuntu (verificado en Fase 1; reverificar)
- 🟡 Importar individuales/múltiples/carpetas con diálogos nativos y drag & drop reales
- 🟡 La interfaz no se congela en lotes (trabajo en hilos + eventos con throttling; falta medir en la app)
- ⏳ Logo/iconos derivados, release en GitHub Actions, tag → Release, instalador Inno Setup, compilación solo al final

## Benchmarks (release, 1600×1200 PNG → WebP 800 px, 3 workers)
| Lote | Tiempo | Img/s | Pico RSS |
|---|---|---|---|
| 10 | 0,7 s | 14,7 | 149 MB |
| 100 | 6,2 s | 16,2 | 328 MB |
| 1000 | 60,7 s | 16,5 | 371 MB |

Memoria estable (crece solo ~40 MB de 100 a 1000). Repetir: `cargo test --release bench_lotes -- --ignored --nocapture`.
