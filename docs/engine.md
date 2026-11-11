# Motor de imagen

**Decisión:** libvips, vía el crate `libvips` **=1.6.1** (fijado), detrás del trait `ImageEngine` (`src-tauri/src/imaging/engine.rs`). Cambiar de motor = implementar el trait; el pipeline, el sistema de archivos y la UI no cambian.

## Por qué 1.6.1 y no 2.x
Los crates `libvips` ≥1.7 generan propiedades de libvips ≥8.16 (`exact`, `input-profile`, `keep` con `gainmap`) y fallan en tiempo de ejecución contra libvips 8.15 (Ubuntu 24.04) con `no property named ...`. 1.6.1 usa la API clásica (`strip`, etc.), que sigue siendo válida en 8.15 y posteriores. Comprobado con tests contra 8.15.1.

**Limitación conocida:** con 1.6.1 la política de metadatos es todo o nada (`Strip` elimina; `KeepIcc` y `KeepAll` conservan todo). Conservar solo ICC queda para post-MVP (spec §3.2); la UI solo ofrece Eliminar/Conservar.

## Pipeline (`imaging/pipeline.rs`)
validar → resolver destino (`filesystem::resolve_output`) → codificar a temporal en la carpeta destino → publicar → resultado.
- Orientación EXIF corregida antes de dimensionar.
- Sin conservar metadatos: perfil ICC incrustado → sRGB para no alterar colores.
- Alfa → JPEG: se aplana sobre blanco. PNG/WebP conservan alfa.
- Resize: `Contain` (cabe en la caja, puede ampliar), `Fit` (cabe, nunca amplía), `Cover` (llena y recorta al centro).
- Límites: 250 M píxeles de entrada, 100 000 px por lado de salida.
- Escritura: temporal `.pixora-<pid>-<n>.tmp.<ext>` + `hard_link` (Rename/Skip, nunca pisa) o `rename` (Overwrite). El temporal se borra siempre (Drop). Nunca se escribe sobre el archivo de origen; sobrescribir exige confirmación explícita.

## Dependencia nativa
Requiere libvips ≥ 8.14 con soporte JPEG, PNG y WebP. Desarrollo Ubuntu: `sudo apt install libvips-dev`. Windows/Linux empaquetado: Fase 7 (DLL junto al exe / dependencia del `.deb`; licencia LGPL: incluir avisos).

## Pruebas
`cargo test` genera sus fixtures en código (PNG con ruido, alfa, JPEG con EXIF inyectado, archivo corrupto) en directorios temporales; no hay binarios versionados.
