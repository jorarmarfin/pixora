# PIXORA — Especificación funcional y técnica

> **Estado:** propuesta de implementación / fuente de verdad inicial del repositorio  
> **Versión del documento:** 1.0 — 2026-11-10  
> **Producto:** Pixora  
> **Tipo:** aplicación de escritorio para procesamiento de imágenes  
> **Sistema de desarrollo principal:** Ubuntu Linux  
> **Plataformas objetivo:** Linux y Windows (macOS, fase futura)

---

## 1. Visión del producto

Pixora será una aplicación de escritorio **offline-first** para convertir, comprimir, optimizar y transformar imágenes individuales o por lotes, con una interfaz moderna, rápida y accesible. Su propósito es evitar tener que subir archivos a servicios externos y simplificar operaciones repetitivas.

**Principios:**

1. Privacidad: procesamiento local por defecto, sin enviar imágenes a servidores.
2. No destructivo: jamás sobrescribir originales por defecto.
3. Transparencia: mostrar tamaño original, final, ahorro y errores por archivo.
4. Rendimiento: procesar lotes con paralelismo acotado y sin bloquear la interfaz.
5. Portabilidad: desarrollo Linux, distribución Linux/Windows.
6. Extensibilidad: operaciones modeladas como tareas de un motor reutilizable.
7. La **compilación y distribución son la última fase** del plan, después de las pruebas.

### Fuera de alcance inicial

- Editor completo de capas tipo Photoshop.
- Servicios cloud obligatorios, cuentas o suscripciones.
- IA generativa o eliminación automática de fondos en el MVP.
- Edición destructiva directa de originales.

## 2. Tecnologías y decisiones

| Componente | Elección | Motivo |
|---|---|---|
| Shell desktop | **Tauri 2** | Ventanas nativas y comunicación frontend–Rust |
| Interfaz | **React + TypeScript + Vite** | Desarrollo modular y conocido |
| Estilos | **Tailwind CSS + shadcn/ui** | UI consistente y componentes accesibles |
| Estado UI | **Zustand** | Estado liviano; cola de procesos reflejada desde Rust |
| Backend local | **Rust** | Gestión de archivos, procesos, trabajo concurrente |
| Motor principal | **libvips** (binding Rust evaluado y fijado por compatibilidad) | Operaciones eficientes sobre imágenes |
| Motor auxiliar | **ImageMagick**, solo si una operación lo necesita | Cobertura adicional de formatos/operaciones |
| Configuración | JSON local | Preferencias y presets sencillos |
| Persistencia opcional | SQLite | Historial persistente y trabajos reanudables (fase posterior) |
| Pruebas frontend | Vitest + Testing Library | Componentes y lógica UI |
| Pruebas backend | `cargo test` + fixtures | Conversión, validaciones y cola |
| Automatización | GitHub Actions | Tests y distribución |
| Instalador Windows | **Inno Setup (ISCC.exe)** | Instalador `.exe` propio |

**Importante:** libvips necesita librerías nativas empaquetadas o instaladas correctamente. Definir explícitamente la estrategia de linking, DLLs y licencias antes de la fase de release. No suponer que el binario Rust incluye automáticamente libvips. Si la combinación de bindings / licencias / distribución lo exige, encapsular el adaptador para poder sustituir el motor sin reescribir las funcionalidades.

## 3. Alcance funcional

### 3.1 MVP — obligatorio

- Importar una o varias imágenes mediante selector de archivos.
- Importar una carpeta y opcionalmente recorrer subcarpetas.
- Drag & drop de archivos o carpetas.
- Vista previa, nombre, dimensiones, formato, peso y ruta de entrada.
- Convertir entre **JPEG, PNG, WebP**; **AVIF** únicamente cuando el códec empaquetado esté validado en ambos sistemas.
- Opciones de calidad y compresión según formato, diferenciando **lossy** y **lossless**.
- Redimensionar por ancho, alto, porcentaje, ajuste proporcional y modo de ajuste (*contain*, *cover* o *fit*).
- Corregir orientación EXIF antes de dimensionar cuando corresponda.
- Procesar archivo individual o lote completo.
- Elegir directorio de salida y sufijo/nomenclatura del nombre de archivo.
- Evitar sobreescrituras: renombrar con sufijo incremental, omitir o solicitar confirmación explícita.
- Barra de progreso global, estados por archivo y opción de cancelar.
- Resumen final: procesados, correctos, fallidos, omitidos, tamaño original y final, ahorro absoluto y porcentual.
- Manejo de archivos corruptos, no soportados, sin permisos y destino sin espacio.
- Presets locales: Web optimizada, Miniaturas, Máxima calidad, Personalizado.
- Preferencias persistentes (carpeta por defecto, tema, último preset).
- Interfaz en español; estructura preparada para i18n.
- Botón para abrir carpeta de destino al finalizar.
- Totalmente funcional sin conexión a Internet.

### 3.2 Funciones posteriores al MVP

- AVIF de producción y formatos adicionales (TIFF, BMP, GIF estático, ICO).
- PNG con reducción de paleta; tratamiento configurable del canal alfa.
- Marca de agua textual o imagen.
- Rotar, voltear, recortar y agregar márgenes/fondo.
- Renombrado masivo configurable.
- Eliminación optativa de metadatos, preservación optativa de perfil ICC.
- Comparación antes/después con zoom.
- Detección de duplicados por hash y similitud perceptual (algoritmos separados).
- Historial local y exportación CSV/JSON de resultados.
- Presets adicionales para redes sociales, tiendas y sitios web.
- Procesamiento vía menú contextual/CLI, si se justifica.

### 3.3 Restricciones por formato

| Formato | Consideración |
|---|---|
| JPEG | No tiene transparencia; definir fondo para entradas con alfa |
| PNG | Normalmente sin pérdida; "calidad" no equivale a JPEG |
| WebP | Soportar variante con pérdida y sin pérdida |
| AVIF | Validar disponibilidad real del codificador en cada paquete |
| SVG | Si se incorpora, requiere rasterización y política de seguridad |
| GIF animado | No prometer preservación de animación sin pipeline específico |
| ICC/EXIF | Definir política de orientación y preservación/eliminación |

## 4. Interfaz y experiencia de usuario

### Distribución base

1. **Barra lateral:** Convertir, Comprimir, Redimensionar, Herramientas, Presets, Ajustes.
2. **Área principal:** zona de arrastrar/soltar y listado de imágenes con miniaturas.
3. **Panel de ajustes:** formato destino, calidad, dimensiones, metadatos, carpeta de salida.
4. **Zona de procesamiento:** inicio, pausa (si está implementada), cancelar, progreso y estadísticas.
5. **Pantalla de resultados:** archivos correctos/errores, ahorro y acceso a carpeta.

### Estados de la tarea

`queued -> validating -> processing -> completed | failed | cancelled | skipped`

Las actualizaciones de progreso se emitirán mediante eventos Tauri desde Rust. La UI **no** debe procesar los píxeles de un lote en el hilo principal de React.

### Accesibilidad

Navegación por teclado, etiquetas visibles, contraste razonable, estados de error claros y soporte de modo claro/oscuro.

### Marca y logo

El usuario colocará **el archivo del logo en la raíz del repositorio**. Definir el nombre canónico como `logo.png` (PNG cuadrado recomendado, alta resolución, idealmente 1024 × 1024 y transparencia), o adaptar la ruta si se suministra otro nombre. En la fase de empaquetado generar derivados desde esa fuente hacia `src-tauri/icons/` y recursos de Inno Setup (`.ico`, según las necesidades); **no asumir que el archivo existe durante las primeras fases**. Si no está, usar iconos temporales de desarrollo sin bloquear `./run.sh`.

## 5. Arquitectura interna

```text
src/
  app/                  # Inicialización, rutas, providers
  components/           # Componentes UI generales
  features/
    import/              # Selector, drag & drop, tabla de imágenes
    convert/             # Configuración de formatos
    optimize/            # Compresión y opciones
    resize/              # Geometría y presets
    jobs/                # Progreso y resultados
    settings/            # Preferencias
  stores/                # Zustand
  lib/                   # Cliente de comandos/eventos Tauri, validaciones
  types/                 # DTO compartidos (tipos TS)
src-tauri/
  src/
    main.rs
    lib.rs
    commands/             # API Tauri
    domain/               # Modelos y reglas
    jobs/                 # Scheduler, cancelación, progreso
    imaging/              # Motor y adapters
      engine.rs           # Trait de procesamiento
      vips.rs             # Adaptador libvips
      imagemagick.rs      # Adaptador secundario (si se activa)
    filesystem/           # Validación de rutas, colisiones y escritura segura
    settings/             # Preferencias
  icons/                  # Iconos derivados del logo raíz
  tauri.conf.json
scripts/
  check-env.sh
  generate-icons.sh
  package-windows.ps1
installer/
  pixora.iss              # Script Inno Setup
.github/workflows/
  ci.yml
  release.yml
fixtures/                  # Imágenes pequeñas para pruebas
run.sh                    # Entrada única desarrollo Linux
logo.png                  # Suministrado por el usuario; no obligatorio al inicio
README.md
PIXORA_SPEC.md
```

### Contrato de motor

El adaptador de imagen recibirá un trabajo tipado e independiente de la UI:

```text
ImageJob {
  id, input_path, output_directory,
  output_format, quality_mode, quality_value,
  resize_options, metadata_policy,
  naming_policy, conflict_policy
}
```

Producirá `JobResult` con dimensiones, rutas, bytes de entrada/salida, duración, estado y error estructurado. No guardar rutas sensibles en logs por defecto.

### Seguridad de archivos

- Canonicalizar/verificar rutas; proteger contra conflictos entre origen y destino.
- Escribir en archivo temporal del mismo directorio y renombrar atómicamente cuando el sistema lo permita.
- Eliminar temporales incompletos al fallar o cancelar.
- Usar límites de dimensiones/píxeles y concurrencia para evitar agotamiento de recursos.
- Evitar ejecutar comandos de shell con rutas concatenadas; usar APIs de proceso con argumentos separados.
- Restringir las capabilities/permisos de Tauri al mínimo.
- Tratar imágenes y metadatos como archivos potencialmente maliciosos; mantener actualizados los decodificadores.

## 6. Modelo de concurrencia y rendimiento

- Cola controlada por Rust, con concurrencia configurable y conservadora (p. ej., 2–4 trabajos simultáneos de inicio; afinar con pruebas).
- Separar validación de lectura, conversión y escritura.
- No mantener todas las imágenes originales decodificadas en RAM.
- Eventos de progreso limitados en frecuencia para no saturar React.
- Cancelación cooperativa: no iniciar nuevos trabajos, y cancelar los activos donde el motor lo permita.
- Evitar prometer pausa/reanudación real hasta implementarlas y probarlas.
- Establecer benchmarks con corpus de 10, 100 y 1000 imágenes de tamaños variados.

**Objetivos de calidad, no promesas de rendimiento:** interfaz responsiva durante lotes; utilización de memoria estable; salidas verificadas; progreso sin bloqueos.

## 7. Flujo de desarrollo local Linux

### Requisitos (Ubuntu)

- Node.js LTS y npm, versiones fijadas en `.nvmrc` / `package.json`.
- Rust estable (`rustup`, `cargo`).
- Prerrequisitos Linux de Tauri 2 (WebKitGTK, librerías de desarrollo, `pkg-config`, etc.), documentados según versión de Ubuntu.
- libvips y sus cabeceras / `pkg-config`, si se compila con linking dinámico.

### Archivo `run.sh` — **obligatorio desde la fase 1**

Ubicación: **raíz del repositorio**. Debe ser ejecutable (`chmod +x run.sh`) y arrancar la aplicación en **modo desarrollo**, sin generar instalador ni hacer `tauri build` de producción.

Comportamiento requerido:

1. Verificar Linux, Node/npm, Cargo/Rust y dependencias críticas.
2. Mostrar instrucciones de instalación si falta algo, con un error explícito.
3. Instalar dependencias npm solo si falta `node_modules` (o indicar `npm ci` si hay desajuste de lockfile).
4. Ejecutar `npm run tauri dev` mediante `exec` para propagar señales/salida.
5. Permitir argumentos adicionales cuando sea conveniente.
6. Nunca ejecutar `sudo`, borrar archivos del usuario ni instalar dependencias del SO automáticamente.

**Implementación inicial sugerida:**

```bash
#!/usr/bin/env bash
set -Eeuo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "Pixora: run.sh está pensado para desarrollo en Linux." >&2
  exit 1
fi

for cmd in node npm cargo rustc pkg-config; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Pixora: falta '$cmd'. Revisa README.md (requisitos Ubuntu)." >&2
    exit 1
  fi
done

# Si libvips es un requisito dinámico del motor seleccionado:
if ! pkg-config --exists vips; then
  echo "Pixora: libvips no está disponible mediante pkg-config." >&2
  echo "Instala las dependencias libvips de desarrollo indicadas en README.md." >&2
  exit 1
fi

if [[ ! -d node_modules ]]; then
  npm ci
fi

exec npm run tauri dev -- "$@"
```

> El control `pkg-config vips` debe ajustarse si finalmente se elige empaquetado estático o un motor alternativo. `run.sh` es el camino rápido de prueba; la CI comprueba versiones y reinstala con `npm ci`.

**Uso esperado:**

```bash
chmod +x run.sh
./run.sh
```

## 8. Estrategia Git y ramas

- `main`: código estable, revisado, integración continua; no genera instaladores públicos automáticamente.
- `feature/*`: funcionalidades aisladas, PR hacia `main`.
- `release`: rama de preparación; cada push ejecuta validación y **build de verificación**, con artefactos descargables de CI pero sin crear automáticamente una versión pública.
- `vX.Y.Z`: **tag semántico** que dispara la publicación de GitHub Release con instaladores finales.

**Decisión:** soportar tanto la rama `release` como los GitHub Releases, evitando publicar una versión definitiva por cada commit. Solo los tags `v*` publican releases oficiales.

### Política recomendada

1. Desarrollar en `feature/*` y fusionar a `main` después de pasar CI.
2. Promover `main` a `release` cuando el producto cumpla criterios de release.
3. Probar el instalador generado por el pipeline de `release`.
4. Crear y publicar tag `v1.0.0` apuntando al commit validado de `release`.
5. El workflow del tag genera y adjunta artefactos en GitHub Releases.
6. El tag no debe publicarse si la versión del `package.json`, `Cargo.toml` y config de Tauri no coincide.

## 9. CI/CD: verificaciones y empaquetado

### `.github/workflows/ci.yml`

Disparadores: `push` a `main`, `pull_request` hacia `main` y `release` (o adaptar a la política del repositorio).

Jobs esperados:

- Frontend: `npm ci`, lint, typecheck, pruebas, `npm run build`.
- Rust/Linux: `cargo fmt --check`, `cargo clippy`, `cargo test`.
- Integración: pruebas con fixtures; no ejecutar UI si el runner es headless sin entorno preparado.
- Caches npm y Cargo para acelerar ejecuciones.

### `.github/workflows/release.yml`

Disparadores:

```yaml
on:
  push:
    branches: [release]
    tags: ['v*']
  workflow_dispatch:
```

Comportamiento:

| Evento | Acción | Publicación |
|---|---|---|
| Push a `release` | Build Windows + Linux, pruebas, empaquetado preliminar | Artefactos de Actions, sin GitHub Release pública |
| `workflow_dispatch` | Build manual para comprobar flujo | Artefactos de Actions |
| Tag `vX.Y.Z` | Validar versión, construir paquetes y crear release | GitHub Release + instaladores |

**Jobs de distribución (fase final):**

- Runner `windows-latest`: generar ejecutable con `npm run tauri build -- --no-bundle` (o equivalente validado con la versión fijada). Preparar directorio de distribución con EXE, DLL nativas, WebView2/runtime según política y recursos. Ejecutar Inno Setup mediante `ISCC.exe installer/pixora.iss` con rutas y versión inyectadas. Publicar `Pixora-Setup-X.Y.Z-x64.exe`.
- Runner `ubuntu-latest`: compilar paquete de Linux (`.deb` y/o AppImage), validando librerías y ejecutando pruebas de instalación básicas donde sea posible. Publicar artefactos de Linux; los nombres finales se definen al comprobar su soporte.
- Usar `actions/upload-artifact` para candidatos y una acción confiable o `gh release` para adjuntar binarios al tag.
- Permisos mínimos de `GITHUB_TOKEN`; `contents: write` solo en job de publicación.
- Evitar secretos en logs y no publicar archivos intermedios innecesarios.
- Firma de código Windows: recomendada antes de distribución comercial; sin firma puede aparecer advertencia SmartScreen.

### Nota importante sobre Inno Setup y Tauri

Tauri 2 incluye por defecto empaquetadores **NSIS** y **MSI**, **no Inno Setup**. En Pixora el instalador de Windows se realizará con un script `.iss` personalizado, sobre el ejecutable y las dependencias compiladas. Esto exige controlar explícitamente qué archivos acompaña al `.exe`; **no basta con empaquetar solo el binario** si hay DLL de libvips, recursos externos u otros runtimes.

El uso de runner Windows es intencional; no se exige compilación cruzada Windows desde Ubuntu.

### Contrato de `installer/pixora.iss`

- Nombre visible: Pixora.
- AppId persistente (GUID generado una vez cuando se inicialice el proyecto real).
- Versión derivada del tag o metadatos de compilación; nunca hardcodear una versión divergente.
- Arquitectura x64 inicial.
- Icono desde el logo raíz transformado a `.ico` durante el build.
- Instalación por usuario o sistema (elegir, documentar y probar permisos; se recomienda por usuario para inicio).
- Accesos directos en menú Inicio y escritorio opcional.
- Desinstalación completa del binario y archivos distribuidos, sin borrar configuraciones del usuario por defecto.
- Idioma del instalador: español (inglés opcional).
- No descargar ejecutables ni instalar componentes del sistema silenciosamente.
- Incluir dependencias nativas necesarias; comprobar arranque en una instalación limpia de Windows.
- Nombre final del artefacto: `Pixora-Setup-{version}-x64.exe`.

## 10. Fases y orden de ejecución

> **Regla de planificación:** **la compilación de distribución, los instaladores y el release son la última fase**. El modo desarrollo (`./run.sh`) se utiliza desde el principio y no cuenta como empaquetado final.

### Fase 1 — Base del proyecto y ejecución local

**Entregables:** Tauri 2 + React + TS + Tailwind, estructura modular, README inicial, `.gitignore`, `run.sh` funcional, icono temporal.  
**Aceptación:** `./run.sh` abre Pixora en Ubuntu sin generar instaladores.

### Fase 2 — UI, importación y modelo de trabajos

**Entregables:** pantalla principal, drag/drop, importación múltiple, previsualización, metadatos, configuración de destino, cola y estados.  
**Aceptación:** 100 archivos se agregan a la lista sin bloquear la interfaz.

### Fase 3 — Conversión y optimización individual

**Entregables:** abstracción del motor, JPEG/PNG/WebP, calidad, resize, rutas y escritura segura, pruebas unitarias.  
**Aceptación:** convertir una imagen con dimensiones/codificación correctas, sin modificar origen.

### Fase 4 — Lotes y robustez

**Entregables:** concurrencia acotada, progreso, cancelación, reporte de fallos y colisiones de nombres.  
**Aceptación:** lote de 100 imágenes mezcladas con errores controlados y resultados consistentes.

### Fase 5 — Presets y experiencia final

**Entregables:** presets, persistencia local, modo claro/oscuro, accesibilidad, configuración y mejoras de UX.  
**Aceptación:** ajustes sobreviven al reinicio, flujo de conversión completo sin conexión.

### Fase 6 — Pruebas, estabilización y documentación

**Entregables:** suites automatizadas, pruebas de calidad y benchmarks, pruebas de regresión, manual de usuario, changelog y matriz de formatos.  
**Aceptación:** sin defectos bloqueantes; criterios de MVP cumplidos; imágenes verificadas y manejo de errores claro.

### Fase 7 — **Última fase: compilación, instaladores y publicación**

**Entregables:** logo raíz integrado, paquete Linux, empaquetado Windows con Inno Setup, workflows CI/CD, artefactos de `release`, tag y GitHub Release.  
**Aceptación:** instalar, abrir, convertir archivos y desinstalar Pixora en máquinas limpias Linux/Windows; descargar instaladores desde un GitHub Release versionado.

## 11. Criterios de aceptación globales

- [ ] En Ubuntu: `./run.sh` arranca la app en modo desarrollo.
- [ ] No hace falta ejecutar `npm run build` ni generar instaladores para probar cada cambio.
- [ ] Se importan imágenes individuales, múltiples y carpetas.
- [ ] Se convierten los formatos garantizados del MVP con parámetros válidos.
- [ ] La interfaz no se congela al procesar lotes.
- [ ] Cancelar detiene nuevos trabajos y deja salidas consistentes.
- [ ] Los archivos de origen quedan intactos de manera predeterminada.
- [ ] Los errores se muestran por archivo sin abortar necesariamente el lote completo.
- [ ] Los presets y preferencias persisten localmente.
- [ ] Los resultados incluyen ahorro y tamaño en bytes confiables.
- [ ] Las pruebas cubren entradas corruptas, transparencias, EXIF, rutas con espacios y Unicode, conflictos y falta de permisos.
- [ ] El logo original se conserva en la raíz y genera los iconos derivados durante empaquetado.
- [ ] `release` produce artefactos verificables en GitHub Actions.
- [ ] Un tag `vX.Y.Z` produce un GitHub Release con instalador Inno Setup de Windows.
- [ ] El instalador de Windows incluye dependencias y permite instalar/desinstalar.
- [ ] La compilación de distribución se ejecuta solo en la última fase del proyecto.

## 12. Convenciones para implementación con asistentes de IA

1. No saltar de fase sin cumplir los criterios de aceptación de la anterior.
2. Priorizar un MVP funcional en Linux mediante `./run.sh`.
3. No implementar ni activar empaquetado de producción antes de la fase 7.
4. Separar procesamiento real de imágenes y UI; usar comandos y eventos tipados.
5. Acompañar cada nueva operación de casos de prueba con fixtures pequeños.
6. Documentar cualquier cambio de dependencias nativas o formatos soportados.
7. No introducir servicios cloud ni llamadas de telemetría sin aprobación explícita.
8. Conservar el logo fuente que el usuario añadirá a la raíz.
9. Mantener archivos originales seguros; operaciones destructivas requieren confirmación clara.
10. No reportar como completada una función sin prueba o validación reproducible.

## 13. Referencias técnicas

- Tauri 2: https://tauri.app/
- Distribución Windows de Tauri (NSIS / MSI): https://tauri.app/distribute/windows-installer/
- Tauri GitHub Action: https://github.com/tauri-apps/tauri-action
- Inno Setup / compilador `ISCC.exe`: https://jrsoftware.org/ishelp/
- libvips: https://www.libvips.org/
- ImageMagick: https://imagemagick.org/

---

**Resumen de decisión:** desarrollar y probar Pixora en Ubuntu con `./run.sh`; implementar de forma incremental el motor de conversión/compresión por lotes; **solo al final** automatizar los paquetes Linux y el instalador Windows con Inno Setup mediante GitHub Actions, con builds de validación en `release` y publicación oficial por tags `vX.Y.Z`.
