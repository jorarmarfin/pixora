# Manual de usuario — Pixora

Pixora convierte, comprime y redimensiona imágenes por lote, **sin conexión**: tus archivos nunca salen del equipo
y los originales no se modifican.

## Flujo básico
1. **Agregar imágenes**: arrástralas a la ventana, o usa *Agregar imágenes* / *Agregar carpeta* (con o sin subcarpetas).
2. En el panel derecho elige **preset**, formato, calidad, tamaño y **carpeta de salida** (obligatoria).
3. *Convertir todo* procesa el lote; la barra muestra el avance y *Cancelar* detiene los trabajos nuevos
   (los que ya están en curso terminan).
4. Al acabar, el resumen muestra correctas/errores/omitidas, el ahorro total y *Abrir carpeta de destino*.

## Ajustes de salida
| Ajuste | Notas |
|---|---|
| Formato | JPEG, PNG o WebP |
| Calidad | 1–100; PNG siempre es sin pérdida; WebP admite sin pérdida |
| Redimensionar | ancho, alto, porcentaje o caja (contener / cubrir / ajustar); respeta la orientación EXIF |
| Metadatos | quitar (por defecto), conservar ICC o conservar todo* |
| Sufijo | se añade al nombre (`foto-pixora.webp`) |
| Si ya existe | renombrar `(1)`, omitir, o sobrescribir (pide confirmación; nunca pisa el original) |

\* Con el motor actual, "conservar ICC" y "conservar todo" se comportan igual (conservan todos los metadatos).

## Presets
Ver `docs/presets.md`. Tres integrados y los tuyos (*Guardar como preset* cuando cambias algo).

## Errores frecuentes
| Mensaje | Qué hacer |
|---|---|
| Archivo corrupto o ilegible | el archivo está dañado; se omite y el lote sigue |
| La imagen es demasiado grande | supera 250 megapíxeles |
| Sin permisos de escritura / La carpeta de salida no es válida | elige otra carpeta |
| No hay espacio en el destino | libera espacio y reintenta (solo se reprocesan las no completadas) |

## Privacidad
Sin telemetría ni red. La configuración vive en `config.json` en la carpeta de configuración del sistema.
