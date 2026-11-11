# Presets y ajustes

## Persistencia
`config.json` en el directorio de configuración del SO (`app_config_dir`: `~/.config/<id>/` en Linux,
`%APPDATA%` en Windows). Escritura atómica (temporal + rename). Si el archivo está dañado o su `version`
es mayor a la soportada, se renombra a `config.json.bak`, se usan los valores por defecto y la pantalla
Ajustes avisa. Valores fuera de rango se corrigen al cargar (calidad 1–100, concurrencia 1–8, PNG siempre sin pérdida).

Campos: `theme` (`dark`|`light`), `concurrency`, `default_output_dir`, `last_preset_id`, `presets` (solo los del usuario).

## Presets integrados (no editables, no se guardan en disco)
| Preset | Formato | Calidad | Tamaño | Metadatos |
|---|---|---|---|---|
| Web optimizada | WebP | 80 | ancho 1920 px | quitar |
| Miniaturas | WebP | 75 | 320×320 cover | quitar |
| Máxima calidad | WebP | sin pérdida | original | conservar todo |

## Uso
- Panel de ajustes → selector **Preset**: aplica uno; si luego cambias algo pasa a *Personalizado* y aparece
  "Guardar como preset".
- Sección **Presets**: lista, aplica y elimina los propios.
- Sección **Ajustes**: tema, imágenes en paralelo, carpeta de salida por defecto.
- Un preset guarda formato, calidad, redimensionado y metadatos; **no** la carpeta de salida, el sufijo ni la política de conflicto.

## Accesibilidad
Controles con `label`, foco visible (`:focus-visible`), barra de progreso con `role=progressbar`, errores en `role=status/alert`.
Modo claro: el acento rosa usa `#be185d` (≥4.5:1 sobre blanco). Pendiente: el texto blanco sobre el extremo cian del gradiente
(botón activo del menú) no llega a AA en claro; revisar en Fase 6.
