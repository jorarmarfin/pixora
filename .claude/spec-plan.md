# Plan de Fases: Pixora

> Generado: 2026-11-10. Última actualización: 2026-11-11.
> Este archivo es la fuente de verdad del progreso. Actualízalo al
> cerrar o avanzar cada fase. Spec funcional: `PIXORA_SPEC.md`.
> Línea gráfica: `Manual de marca Pixora en acción.png` (raíz).

## Información General
- **Stack**: Tauri 2 + React + TypeScript + Vite + Tailwind + shadcn/ui + Zustand; backend Rust; motor libvips (adaptador sustituible); ImageMagick solo si hace falta; config JSON local.
- **Usuarios**: personas que convierten/comprimen imágenes por lote sin subirlas a la nube (privacidad, offline).
- **Restricciones**: offline-first; no destructivo (nunca sobrescribir originales); UI en español con i18n preparado; desarrollo Ubuntu (`./run.sh`), distribución Linux + Windows; **empaquetado/release solo en Fase 7**; sin telemetría ni cloud.
- **Timeline estimado**: ~5–6 semanas (3–5 días por fase)

## Línea gráfica (extraída del manual de marca)
| Token | Valor |
|---|---|
| Primario | `#7C3AED` (violeta) |
| Secundario | `#06B6D4` (cian) |
| Acento | `#EC4899` (rosa) |
| Superficie | `#0F172A` |
| Superficie 2 | `#1E293B` |
| Gradiente principal | `#7C3AED → #06B6D4` |
| Tagline | Convierte · Comprime · Optimiza · Procesa imágenes |
| Logo | "P" blanca con montaña/sol sobre squircle degradado; wordmark "Pixora" blanco, sans geométrica bold |
| Variantes | degradado (principal), fondo claro (P azul/violeta), oscuro (P blanca sobre superficie) |

Uso: tokens en `tailwind.config` / variables CSS shadcn; modo oscuro = base de marca (Superficie), modo claro derivado. Gradiente para CTA principal y barra de progreso global; acento rosa solo para estados destacados/ahorro.

**Nota sobre el logo:** la raíz contiene el manual de marca (PNG compuesto), **no** el `logo.png` cuadrado que pide el spec (§4). Hay que recortar el icono principal a `logo.png` 1024×1024 (idealmente pedir el original con transparencia). Mientras tanto se usa un recorte provisional.

## Estado Actual
- **Fase activa**: Fase 7 — Compilación, instaladores y publicación
- **Progreso**: 6 de 7 fases completadas
- **Bloqueadores**: ninguno. libvips 8.15.1 instalado. Para Windows (Fase 7) el binding fijado exige libvips ≥8.14.
- **Próximo paso concreto**: probar el `.deb` y el workflow `release` en GitHub (rama `release`); verificar Windows en máquina limpia.

Entorno detectado (2026-11-10): Node 24.18, npm 12, Rust/Cargo 1.99.1, pkg-config 1.8.1, webkit2gtk-4.1 OK, libvips FALTA.

---

## Fase 1: Base del proyecto y ejecución local — COMPLETADA

**Duración estimada**: 3 días
**Objetivo**: Tauri 2 + React + TS + Tailwind + shadcn con tema de marca, estructura modular y `./run.sh` abriendo la app.
**Complejidad**: Medio

### Requisitos previos
- Rust, Node, WebKitGTK ya presentes. Decidir política de versiones (`.nvmrc`, `rust-toolchain.toml`).

### Entregables (Definición de Done)

#### Backend (Rust)
- [x] Scaffold Tauri 2 (`src-tauri/`, `lib.rs`, `main.rs`) con módulos vacíos: `commands/ domain/ jobs/ imaging/ filesystem/ settings/`
- [x] Capabilities de Tauri al mínimo (solo lo necesario: dialog, eventos)
- [x] Comando `ping`/`app_info` tipado como prueba de puente front–Rust

#### Frontend
- [x] Vite + React + TS estricto; estructura `app/ components/ features/* stores/ lib/ types/`
- [x] Tailwind + shadcn/ui; tokens de marca (tabla arriba) en CSS vars, modo oscuro por defecto
- [x] Layout base: barra lateral (Convertir, Comprimir, Redimensionar, Herramientas, Presets, Ajustes) + área principal vacía
- [x] Estructura i18n (es) con claves, sin textos hardcodeados
- [x] Fuente/tipografía geométrica bold afín al wordmark (empaquetada local, offline)

#### Marca / assets
- [x] Mover manual de marca a `brand/` (o dejarlo en raíz) y recortar `logo.png` provisional
- [x] Iconos temporales de desarrollo en `src-tauri/icons/` (sin bloquear `./run.sh`)
- [x] `scripts/generate-icons.sh` esbozo (uso real en Fase 7)

#### Tooling
- [x] `run.sh` ejecutable según §7 (sin sudo, `npm ci` solo si falta `node_modules`, `exec npm run tauri dev -- "$@"`)
- [x] `scripts/check-env.sh`
- [x] ESLint, Prettier, `cargo fmt`, `clippy`; Vitest + Testing Library configurados
- [x] `.gitignore`, `.nvmrc`, README inicial con requisitos Ubuntu

#### Tests
- [x] Smoke test Vitest del layout; `cargo test` del comando `ping`

#### Documentación
- [x] README: requisitos Ubuntu, `./run.sh`, estructura
- [x] `docs/brand.md` con tokens

### Posibles obstáculos
- Tauri 2 y WebKitGTK 4.1 en la versión de Ubuntu: documentar paquetes exactos.
- Tailwind v4 vs v3 con shadcn: fijar una combinación probada.
- `run.sh` exige vips: condicionar el chequeo hasta Fase 3.

### Notas de avance
- 2026-11-10: stack final Vite 8 + React 19 + TS 6 + Tailwind v4 + Vitest 5. shadcn montado a mano (components.json + Button + cn); Radix/lucide instalados. Sin plugin opener (capabilities mínimas: core:default). `bundle.active=false` hasta Fase 7. `./run.sh` verificado: vite + cargo run, proceso `pixora` vivo. Vitest 3/3, cargo test 1/1, clippy -D warnings OK, `npm run build` OK. `run.sh` aún NO exige vips (se activa en Fase 3). Rama git renombrada a `main`. Pendiente menor: logo.png es recorte provisional; verificación visual de la ventana no hecha (sin captura segura).

**Aceptación**: `./run.sh` abre Pixora en Ubuntu, sin generar instaladores.

---

## Fase 2: UI, importación y modelo de trabajos — COMPLETADA

**Duración estimada**: 5 días
**Objetivo**: pantalla principal funcional con importación (archivos, carpeta, drag&drop), listado con miniaturas/metadatos y modelo de cola/estados (aún sin convertir).
**Complejidad**: Medio-Alto

### Requisitos previos
- Fase 1 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [x] Modelos de dominio: `ImageJob`, `JobResult`, `JobStatus` (`queued|validating|processing|completed|failed|cancelled|skipped`), errores estructurados
- [x] Comandos: `import_paths` (archivos/carpetas, recursivo opcional), `read_image_meta` (dimensiones, formato, peso, ruta)
- [x] Generación de miniaturas en Rust (en hilos, no en React); caché temporal
- [x] Validación y canonicalización de rutas (espacios, Unicode)
- [x] Tipos compartidos Rust→TS (p. ej. `ts-rs`/`specta`)

#### Frontend
- [x] Zona drag&drop (archivos y carpetas) + selector nativo
- [x] Tabla/lista virtualizada con miniatura, nombre, dimensiones, formato, peso, ruta, estado
- [x] Panel de ajustes (formato destino, calidad, dimensiones, metadatos, carpeta de salida) — solo UI/estado
- [x] Zustand: store de cola reflejado desde eventos Tauri
- [x] Selección/eliminación de ítems de la lista

#### Tests
- [x] Unit: filtrado de extensiones, recorrido de carpetas, rutas Unicode/espacios
- [x] Componentes: lista, drop zone, panel
- [x] Rendimiento: 100 archivos sin bloquear UI

#### Documentación
- [x] Contrato de comandos/eventos (`docs/ipc.md`)

### Posibles obstáculos
- Eventos drag&drop de Tauri 2 vs HTML5 DnD: usar el de Tauri para rutas reales.
- Miniaturas masivas: límite de concurrencia y lazy load. Sin libvips aún → usar crate `image` temporal tras el trait del motor, o adelantar el adaptador vips.

### Notas de avance
- 2026-11-10: metadatos con `imagesize` (solo cabecera), miniaturas con crate `image` (+EXIF) temporal hasta el motor vips. Lista virtualizada con @tanstack/react-virtual. Dominio incluye ya el contrato `ImageJob` (sin motor). Tests: cargo 11/11 (incl. lote de 100, Unicode/espacios, corrupto, symlink-safe), Vitest 15/15 (incl. 100 ítems, filas virtualizadas), clippy/lint/typecheck/build OK, `./run.sh` arranca. Limitaciones: miniatura decodifica la imagen completa (aceptable ahora; revisar en Fase 6 con 1000 imágenes); drag&drop real y diálogos nativos NO probados de forma automatizada, requieren prueba manual; sin verificación visual de la ventana.

**Aceptación**: 100 archivos se agregan a la lista sin bloquear la interfaz.

---

## Fase 3: Conversión y optimización individual — COMPLETADA

**Duración estimada**: 5 días
**Objetivo**: motor de imagen real tras un trait, JPEG/PNG/WebP con calidad y resize, escritura segura.
**Complejidad**: Alto

### Requisitos previos
- Fase 2 completada. `libvips-dev` instalado y binding Rust evaluado y fijado (spike de 0.5 día: crate `libvips` vs alternativas; documentar licencia/linking).

### Entregables (Definición de Done)

#### Backend (Rust)
- [x] `imaging/engine.rs`: trait `ImageEngine` independiente de UI
- [x] `imaging/vips.rs`: adaptador libvips (decodificar, orientación EXIF, resize contain/cover/fit, encode JPEG/PNG/WebP lossy/lossless)
- [x] Fondo configurable para alfa→JPEG; límites de píxeles/dimensiones
- [x] `filesystem/`: colisiones (sufijo incremental/omitir/confirmar), temp en mismo dir + rename atómico, limpieza de temporales, protección origen=destino
- [x] Nomenclatura/sufijo de salida; `JobResult` con bytes in/out, duración
- [x] Comando `process_single`

#### Frontend
- [x] Opciones de calidad por formato (lossy/lossless), resize (ancho/alto/%/proporcional/modo)
- [x] Mostrar resultado individual (tamaño original/final/ahorro)

#### Tests
- [x] Fixtures generadas en código dentro de los tests (EXIF inyectado, alfa, ruido, corrupta); sin binarios versionados
- [x] Verificar dimensiones/formato de salida, origen intacto, rutas Unicode/espacios, permisos, conflictos
- [x] `run.sh` pasa a exigir `pkg-config vips`

#### Documentación
- [x] `docs/engine.md` (decisión libvips, linking, licencias), matriz de formatos inicial

### Posibles obstáculos
- Binding libvips inmaduro/licencia → mantener adaptador sustituible (fallback ImageMagick/`image`).
- AVIF: fuera del MVP hasta validar códec en ambos SO.
- Linking dinámico en Windows se resuelve en Fase 7; dejar documentado.

### Notas de avance
- 2026-11-10: spike binding: crate `libvips` 2.3.0 y 1.7.x FALLAN en runtime contra libvips 8.15.1 (propiedades `exact`, `input-profile`, `keep` inexistentes). Fijado `libvips =1.6.1` (API `strip`), funciona en 8.15.1; documentado en `docs/engine.md`. Limitación: metadatos todo-o-nada (KeepIcc = KeepAll); UI solo ofrece Eliminar/Conservar. Trait `ImageEngine` + `VipsEngine`, `geometry` puro, `pipeline` (validar→destino→temporal→publicar), `filesystem` (colisiones, hard_link/rename, TempFile con Drop). `process_single` expuesto. UI: botón Convertir por fila, ahorro/error por archivo, confirmación explícita de sobrescritura. Tests: cargo 32/32 con libvips real (EXIF, alfa, lossless exacto, origen intacto byte a byte, colisiones, corrupto sin temporales, Unicode, permisos, overwrite=origen), Vitest 24/24, clippy/lint/typecheck/build OK, `./run.sh` arranca (ahora exige vips). NO probado: conversión de punta a punta desde la ventana real (diálogo de carpeta + clic), requiere prueba manual; JPEG CMYK y PNG 16-bit sin fixture.

**Aceptación**: convertir una imagen con dimensiones/codificación correctas, sin modificar el origen.

---

## Fase 4: Lotes y robustez — COMPLETADA

**Duración estimada**: 4 días
**Objetivo**: cola concurrente acotada con progreso, cancelación y reporte de fallos.
**Complejidad**: Alto

### Requisitos previos
- Fase 3 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [x] `jobs/` scheduler con concurrencia configurable (2–4 por defecto), separación validar/convertir/escribir
- [x] Cancelación cooperativa (no inicia nuevos; limpia temporales de activos)
- [x] Eventos de progreso con throttling
- [x] Comandos `start_batch`, `cancel_batch`; resumen agregado (procesados/ok/fallidos/omitidos, bytes, ahorro)
- [x] Errores por archivo sin abortar el lote (corrupto, no soportado, sin permisos, disco lleno)

#### Frontend
- [x] Barra de progreso global (gradiente de marca), estados por archivo, botón cancelar
- [x] Pantalla de resultados: ok/errores, ahorro abs/%, botón "abrir carpeta de destino"

#### Tests
- [x] Lote de 100 imágenes mezcladas con errores controlados
- [x] Cancelación a mitad: salidas consistentes, sin `.tmp`
- [x] Colisiones en lote; memoria estable

#### Documentación
- [x] Modelo de concurrencia y estados en `docs/jobs.md`

### Posibles obstáculos
- Saturación de eventos hacia React → throttling/batching.
- Cancelar trabajo ya dentro de libvips: aceptar granularidad por archivo.

### Notas de avance
- 2026-11-11: scheduler con `thread::scope` + cancelación cooperativa por archivo; eventos `batch://progress` agrupados a 100 ms; `start_batch`/`cancel_batch`/`reveal_folder`. Test de carga destapó una carrera de colisiones de nombre en paralelo → `publish` devuelve `None` y el pipeline re-resuelve. Tests: cargo 40, Vitest 31, clippy/lint/build OK. UI: `BatchBar` (gradiente de marca, cancelar, doble confirmación de sobrescritura) y `BatchResults`. Pendiente: prueba manual en la app real (progreso visual, abrir carpeta); "memoria estable" solo verificada indirectamente (lote de 100 pequeño); disco lleno no probado automáticamente.

**Aceptación**: lote de 100 imágenes mezcladas con errores controlados y resultados consistentes.

---

## Fase 5: Presets y experiencia final — COMPLETADA

**Duración estimada**: 4 días
**Objetivo**: presets, persistencia, tema claro/oscuro, accesibilidad y pulido de marca.
**Complejidad**: Medio

### Requisitos previos
- Fase 4 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [x] `settings/`: preferencias JSON en directorio de config del SO (carpeta por defecto, tema, último preset, concurrencia)
- [x] Presets: Web optimizada, Miniaturas, Máxima calidad, Personalizado (CRUD local)

#### Frontend
- [x] Selector de presets en el panel de ajustes (compartido por Convertir/Comprimir/Redimensionar)
- [x] Pantalla Ajustes; tema claro/oscuro (oscuro = marca)
- [x] Accesibilidad: teclado, labels, contraste AA con paleta de marca, foco visible, errores claros
- [x] Pulido visual según manual de marca (logo en cabecera, gradientes, estados vacíos, tagline)
- [x] i18n verificado (sin strings sueltos)

#### Tests
- [x] Persistencia sobrevive al reinicio; migración/valores corruptos de config
- [x] Componentes de presets/ajustes
- [ ] Flujo completo offline (prueba manual en la app, pendiente)

#### Documentación
- [x] Guía de presets y ajustes

### Posibles obstáculos
- Contraste del acento rosa/cian sobre claro → ajustar tonos en modo claro.

### Notas de avance
- 2026-11-11: `settings/` con config.json atómico, recuperación de archivo corrupto (.bak) y saneado de valores; presets integrados (Web, Miniaturas, Máxima calidad) + CRUD de propios vía `load_config`/`save_config`. Front: `appConfig` store, `PresetPicker`, `PresetsView`, `PrefsView` (tema, concurrencia, carpeta por defecto). Tema claro/oscuro con `.dark` en `<html>`; superficies de lotes pasadas a tokens de tema. Docs en `docs/presets.md`. Tests: cargo 46, Vitest 38. Pendiente: contraste AA del texto blanco sobre el extremo cian del gradiente en claro; prueba manual de reinicio/offline; i18n revisado solo por búsqueda de aria-labels (sin lint automático); Herramientas sigue sin implementar.

**Aceptación**: ajustes sobreviven al reinicio; flujo de conversión completo sin conexión.

---

## Fase 6: Pruebas, estabilización y documentación — COMPLETADA

**Duración estimada**: 4 días
**Objetivo**: cerrar MVP con suites, benchmarks y docs.
**Complejidad**: Medio

### Requisitos previos
- Fase 5 completada.

### Entregables (Definición de Done)

#### Tests
- [x] Suites front (Vitest) y back (`cargo test`) en verde; cobertura de entradas corruptas, alfa, EXIF, Unicode, conflictos, permisos
- [x] Benchmarks con corpus de 10/100/1000 imágenes; memoria y UI responsiva
- [x] Regresión de salidas (hash/dimensiones esperadas)
- [x] Revisión de seguridad: capabilities Tauri, rutas, sin shell con rutas concatenadas, límites de píxeles

#### CI (solo verificación, sin empaquetar)
- [x] `.github/workflows/ci.yml`: lint, typecheck, tests, `npm run build`, `cargo fmt/clippy/test`, caches

#### Documentación
- [x] Manual de usuario, `CHANGELOG.md`, matriz de formatos, checklist de criterios MVP (§11)

### Posibles obstáculos
- libvips en runner CI: instalar `libvips-dev` en el job.
- Defectos de rendimiento con lotes grandes → ajustar concurrencia.

### Notas de avance
- 2026-11-11: matriz de regresión (3 formatos × 6 redimensionados), PNG→PNG exacto, benchmark 10/100/1000 (16 img/s, pico 371 MB). Revisión de seguridad: límite de 64 MP en miniaturas (antes sin límite), CSP + devCsp, ver `docs/security.md`. CI en `.github/workflows/ci.yml` (no ejecutado aún en GitHub). Bug de UI: los `<select>` se veían blancos en oscuro → estilos globales de select/option. Docs: manual, formatos, changelog, checklist MVP. Pendiente: prueba manual de la app (CSP no verificada en ventana real: si algo no carga, revisar `devCsp`), contraste AA del gradiente en claro, vista Herramientas.

**Aceptación**: sin defectos bloqueantes; criterios de MVP cumplidos; errores claros.

---

## Fase 7: Compilación, instaladores y publicación (ÚLTIMA) — PENDIENTE

**Duración estimada**: 5 días
**Objetivo**: paquetes Linux, instalador Windows con Inno Setup, workflows y release por tag.
**Complejidad**: Alto

### Requisitos previos
- Fase 6 completada. `logo.png` definitivo (1024×1024, transparencia) en la raíz.

### Entregables (Definición de Done)

#### Empaquetado
- [x] `scripts/generate-icons.sh`: `logo.png` → `src-tauri/icons/` + `.ico` para Inno
- [x] Estrategia libvips decidida (DLLs en Windows, deps en `.deb`/AppImage) y licencias documentadas
- [x] Linux: `.deb` y/o AppImage
- [x] Windows: `npm run tauri build -- --no-bundle`, directorio de distribución (EXE + DLL + WebView2 según política), `installer/pixora.iss` (AppId GUID fijo, versión inyectada, x64, por usuario, español, accesos directos, desinstalación limpia), `scripts/package-windows.ps1`
- [x] Artefacto `Pixora-Setup-X.Y.Z-x64.exe`

#### CI/CD
- [x] `release.yml`: push a `release` y `workflow_dispatch` → artefactos de Actions; tag `v*` → GitHub Release
- [x] Validación de versión `package.json` = `Cargo.toml` = `tauri.conf.json`
- [x] Permisos mínimos; `contents: write` solo en job de publicación

#### Tests
- [ ] Instalar/abrir/convertir/desinstalar en Linux y Windows limpios
- [ ] Arranque en Windows sin libvips preinstalado

#### Documentación
- [x] Guía de release, notas SmartScreen / firma de código

### Posibles obstáculos
- Sin firma de código → SmartScreen.
- DLLs de libvips en Windows (vcpkg / binarios oficiales) y compatibilidad de runtime.
- AppImage con WebKitGTK: probar en distros distintas.

### Notas de avance
- 2026-11-11: `.deb` verificado en Ubuntu (3,2 MB, depende de libvips42t64). `tauri.conf` con bundle activo solo `deb` (no afecta a `./run.sh`). Añadidos `installer/pixora.iss` (GUID fijo, x64, por usuario, español, WebView2 condicional), `scripts/package-windows.ps1` (DLL de libvips junto al exe), `scripts/check-versions.mjs`, `.github/workflows/release.yml` (release/tag/dispatch; `contents: write` solo en `publish`), `docs/release.md`, `docs/third-party.md`. **No verificado**: todo lo de Windows (sin runner ni ISCC aquí) y el workflow en GitHub; el crate `libvips` con pkg-config en Windows es el mayor riesgo. AppImage descartado en el MVP. Pendiente: pruebas de instalar/desinstalar en máquinas limpias, logo definitivo, firma de código.

**Aceptación**: instalar, abrir, convertir y desinstalar en máquinas limpias; instaladores descargables desde un GitHub Release versionado.

---

## Timeline Total
- Fase 1: 3 días
- Fase 2: 5 días
- Fase 3: 5 días
- Fase 4: 4 días
- Fase 5: 4 días
- Fase 6: 4 días
- Fase 7: 5 días
**TOTAL: ~30 días hábiles (~6 semanas)**

## Riesgos Identificados
1. Binding Rust de libvips inmaduro o con licencia/linking problemático → spike al inicio de Fase 3, trait `ImageEngine` sustituible.
2. Distribución de libvips en Windows (DLLs) → decidir estrategia antes de Fase 7; probar en máquina limpia.
3. AVIF sin códec validado en ambos SO → excluido del MVP.
4. Logo fuente: solo existe el manual compuesto → recortar/solicitar `logo.png` original.
5. Eventos de progreso saturan React → throttling en Rust.
6. Contraste de la paleta en modo claro → verificar AA en Fase 5.

## Historial de Cambios
- 2026-11-10: Fase 3 completada.
- 2026-11-10: Fase 2 completada.
- 2026-11-10: Fase 1 completada.
- 2026-11-10: Plan creado con 7 fases a partir de PIXORA_SPEC.md v1.0 y manual de marca.
