# Plan de Fases: Pixora

> Generado: 2026-11-10. Última actualización: 2026-11-10.
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
- **Fase activa**: Fase 2 — UI, importación y modelo de trabajos
- **Progreso**: 1 de 7 fases completadas
- **Bloqueadores**: `libvips` y cabeceras no instalados (`pkg-config vips` falla). Instalar `libvips-dev` antes de Fase 3 (no bloquea Fase 1–2; `run.sh` solo exigirá vips desde Fase 3). Repo git sin commits.
- **Próximo paso concreto**: modelos de dominio (`ImageJob`, `JobStatus`) + comando `import_paths` y drag&drop con tauri-plugin-dialog.

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

## Fase 2: UI, importación y modelo de trabajos — PENDIENTE

**Duración estimada**: 5 días
**Objetivo**: pantalla principal funcional con importación (archivos, carpeta, drag&drop), listado con miniaturas/metadatos y modelo de cola/estados (aún sin convertir).
**Complejidad**: Medio-Alto

### Requisitos previos
- Fase 1 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [ ] Modelos de dominio: `ImageJob`, `JobResult`, `JobStatus` (`queued|validating|processing|completed|failed|cancelled|skipped`), errores estructurados
- [ ] Comandos: `import_paths` (archivos/carpetas, recursivo opcional), `read_image_meta` (dimensiones, formato, peso, ruta)
- [ ] Generación de miniaturas en Rust (en hilos, no en React); caché temporal
- [ ] Validación y canonicalización de rutas (espacios, Unicode)
- [ ] Tipos compartidos Rust→TS (p. ej. `ts-rs`/`specta`)

#### Frontend
- [ ] Zona drag&drop (archivos y carpetas) + selector nativo
- [ ] Tabla/lista virtualizada con miniatura, nombre, dimensiones, formato, peso, ruta, estado
- [ ] Panel de ajustes (formato destino, calidad, dimensiones, metadatos, carpeta de salida) — solo UI/estado
- [ ] Zustand: store de cola reflejado desde eventos Tauri
- [ ] Selección/eliminación de ítems de la lista

#### Tests
- [ ] Unit: filtrado de extensiones, recorrido de carpetas, rutas Unicode/espacios
- [ ] Componentes: lista, drop zone, panel
- [ ] Rendimiento: 100 archivos sin bloquear UI

#### Documentación
- [ ] Contrato de comandos/eventos (`docs/ipc.md`)

### Posibles obstáculos
- Eventos drag&drop de Tauri 2 vs HTML5 DnD: usar el de Tauri para rutas reales.
- Miniaturas masivas: límite de concurrencia y lazy load. Sin libvips aún → usar crate `image` temporal tras el trait del motor, o adelantar el adaptador vips.

### Notas de avance
- 

**Aceptación**: 100 archivos se agregan a la lista sin bloquear la interfaz.

---

## Fase 3: Conversión y optimización individual — PENDIENTE

**Duración estimada**: 5 días
**Objetivo**: motor de imagen real tras un trait, JPEG/PNG/WebP con calidad y resize, escritura segura.
**Complejidad**: Alto

### Requisitos previos
- Fase 2 completada. `libvips-dev` instalado y binding Rust evaluado y fijado (spike de 0.5 día: crate `libvips` vs alternativas; documentar licencia/linking).

### Entregables (Definición de Done)

#### Backend (Rust)
- [ ] `imaging/engine.rs`: trait `ImageEngine` independiente de UI
- [ ] `imaging/vips.rs`: adaptador libvips (decodificar, orientación EXIF, resize contain/cover/fit, encode JPEG/PNG/WebP lossy/lossless)
- [ ] Fondo configurable para alfa→JPEG; límites de píxeles/dimensiones
- [ ] `filesystem/`: colisiones (sufijo incremental/omitir/confirmar), temp en mismo dir + rename atómico, limpieza de temporales, protección origen=destino
- [ ] Nomenclatura/sufijo de salida; `JobResult` con bytes in/out, duración
- [ ] Comando `process_single`

#### Frontend
- [ ] Opciones de calidad por formato (lossy/lossless), resize (ancho/alto/%/proporcional/modo)
- [ ] Mostrar resultado individual (tamaño original/final/ahorro)

#### Tests
- [ ] `fixtures/` pequeñas: JPEG con EXIF rotado, PNG con alfa, WebP, corrupta, no soportada
- [ ] Verificar dimensiones/formato de salida, origen intacto, rutas Unicode/espacios, permisos, conflictos
- [ ] `run.sh` pasa a exigir `pkg-config vips`

#### Documentación
- [ ] `docs/engine.md` (decisión libvips, linking, licencias), matriz de formatos inicial

### Posibles obstáculos
- Binding libvips inmaduro/licencia → mantener adaptador sustituible (fallback ImageMagick/`image`).
- AVIF: fuera del MVP hasta validar códec en ambos SO.
- Linking dinámico en Windows se resuelve en Fase 7; dejar documentado.

### Notas de avance
- 

**Aceptación**: convertir una imagen con dimensiones/codificación correctas, sin modificar el origen.

---

## Fase 4: Lotes y robustez — PENDIENTE

**Duración estimada**: 4 días
**Objetivo**: cola concurrente acotada con progreso, cancelación y reporte de fallos.
**Complejidad**: Alto

### Requisitos previos
- Fase 3 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [ ] `jobs/` scheduler con concurrencia configurable (2–4 por defecto), separación validar/convertir/escribir
- [ ] Cancelación cooperativa (no inicia nuevos; limpia temporales de activos)
- [ ] Eventos de progreso con throttling
- [ ] Comandos `start_batch`, `cancel_batch`; resumen agregado (procesados/ok/fallidos/omitidos, bytes, ahorro)
- [ ] Errores por archivo sin abortar el lote (corrupto, no soportado, sin permisos, disco lleno)

#### Frontend
- [ ] Barra de progreso global (gradiente de marca), estados por archivo, botón cancelar
- [ ] Pantalla de resultados: ok/errores, ahorro abs/%, botón "abrir carpeta de destino"

#### Tests
- [ ] Lote de 100 imágenes mezcladas con errores controlados
- [ ] Cancelación a mitad: salidas consistentes, sin `.tmp`
- [ ] Colisiones en lote; memoria estable

#### Documentación
- [ ] Modelo de concurrencia y estados en `docs/jobs.md`

### Posibles obstáculos
- Saturación de eventos hacia React → throttling/batching.
- Cancelar trabajo ya dentro de libvips: aceptar granularidad por archivo.

### Notas de avance
- 

**Aceptación**: lote de 100 imágenes mezcladas con errores controlados y resultados consistentes.

---

## Fase 5: Presets y experiencia final — PENDIENTE

**Duración estimada**: 4 días
**Objetivo**: presets, persistencia, tema claro/oscuro, accesibilidad y pulido de marca.
**Complejidad**: Medio

### Requisitos previos
- Fase 4 completada.

### Entregables (Definición de Done)

#### Backend (Rust)
- [ ] `settings/`: preferencias JSON en directorio de config del SO (carpeta por defecto, tema, último preset, concurrencia)
- [ ] Presets: Web optimizada, Miniaturas, Máxima calidad, Personalizado (CRUD local)

#### Frontend
- [ ] Selector de presets en las vistas Convertir/Comprimir/Redimensionar
- [ ] Pantalla Ajustes; tema claro/oscuro (oscuro = marca)
- [ ] Accesibilidad: teclado, labels, contraste AA con paleta de marca, foco visible, errores claros
- [ ] Pulido visual según manual de marca (logo en cabecera, gradientes, estados vacíos, tagline)
- [ ] i18n verificado (sin strings sueltos)

#### Tests
- [ ] Persistencia sobrevive al reinicio; migración/valores corruptos de config
- [ ] Componentes de presets/ajustes; flujo completo offline

#### Documentación
- [ ] Guía de presets y ajustes

### Posibles obstáculos
- Contraste del acento rosa/cian sobre claro → ajustar tonos en modo claro.

### Notas de avance
- 

**Aceptación**: ajustes sobreviven al reinicio; flujo de conversión completo sin conexión.

---

## Fase 6: Pruebas, estabilización y documentación — PENDIENTE

**Duración estimada**: 4 días
**Objetivo**: cerrar MVP con suites, benchmarks y docs.
**Complejidad**: Medio

### Requisitos previos
- Fase 5 completada.

### Entregables (Definición de Done)

#### Tests
- [ ] Suites front (Vitest) y back (`cargo test`) en verde; cobertura de entradas corruptas, alfa, EXIF, Unicode, conflictos, permisos
- [ ] Benchmarks con corpus de 10/100/1000 imágenes; memoria y UI responsiva
- [ ] Regresión de salidas (hash/dimensiones esperadas)
- [ ] Revisión de seguridad: capabilities Tauri, rutas, sin shell con rutas concatenadas, límites de píxeles

#### CI (solo verificación, sin empaquetar)
- [ ] `.github/workflows/ci.yml`: lint, typecheck, tests, `npm run build`, `cargo fmt/clippy/test`, caches

#### Documentación
- [ ] Manual de usuario, `CHANGELOG.md`, matriz de formatos, checklist de criterios MVP (§11)

### Posibles obstáculos
- libvips en runner CI: instalar `libvips-dev` en el job.
- Defectos de rendimiento con lotes grandes → ajustar concurrencia.

### Notas de avance
- 

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
- [ ] `scripts/generate-icons.sh`: `logo.png` → `src-tauri/icons/` + `.ico` para Inno
- [ ] Estrategia libvips decidida (DLLs en Windows, deps en `.deb`/AppImage) y licencias documentadas
- [ ] Linux: `.deb` y/o AppImage
- [ ] Windows: `npm run tauri build -- --no-bundle`, directorio de distribución (EXE + DLL + WebView2 según política), `installer/pixora.iss` (AppId GUID fijo, versión inyectada, x64, por usuario, español, accesos directos, desinstalación limpia), `scripts/package-windows.ps1`
- [ ] Artefacto `Pixora-Setup-X.Y.Z-x64.exe`

#### CI/CD
- [ ] `release.yml`: push a `release` y `workflow_dispatch` → artefactos de Actions; tag `v*` → GitHub Release
- [ ] Validación de versión `package.json` = `Cargo.toml` = `tauri.conf.json`
- [ ] Permisos mínimos; `contents: write` solo en job de publicación

#### Tests
- [ ] Instalar/abrir/convertir/desinstalar en Linux y Windows limpios
- [ ] Arranque en Windows sin libvips preinstalado

#### Documentación
- [ ] Guía de release, notas SmartScreen / firma de código

### Posibles obstáculos
- Sin firma de código → SmartScreen.
- DLLs de libvips en Windows (vcpkg / binarios oficiales) y compatibilidad de runtime.
- AppImage con WebKitGTK: probar en distros distintas.

### Notas de avance
- 

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
- 2026-11-10: Fase 1 completada.
- 2026-11-10: Plan creado con 7 fases a partir de PIXORA_SPEC.md v1.0 y manual de marca.
