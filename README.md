# Pixora

Aplicación de escritorio **offline-first** para convertir, comprimir, optimizar y transformar imágenes. Spec: [`PIXORA_SPEC.md`](PIXORA_SPEC.md). Plan: [`.claude/spec-plan.md`](.claude/spec-plan.md).

## Requisitos (Ubuntu)

- Node.js LTS (ver `.nvmrc`) y npm
- Rust estable (`rustup`), `pkg-config`
- Tauri 2: `sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev`
- libvips (obligatorio desde la Fase 3): `sudo apt install libvips-dev`

Comprobar entorno: `scripts/check-env.sh`

## Desarrollo

```bash
./run.sh          # modo desarrollo (tauri dev); no genera instaladores
npm test          # Vitest
npm run lint && npm run typecheck
cd src-tauri && cargo test && cargo clippy -- -D warnings
```

## Compilar para distribución

Los instaladores se generan **en cada sistema operativo** (el enlazado con libvips en Windows usa MSVC y no se puede hacer desde Linux).

### Linux (`.deb`)

```bash
./scripts/build-linux.sh               # o: npm run build:linux
./scripts/build-linux.sh --skip-tests  # sin npm test / cargo test
```

Qué hace: comprueba el entorno y libvips, valida que las versiones coincidan, corre los tests y ejecuta `tauri build --bundles deb`.
Salida: `src-tauri/target/release/bundle/deb/Pixora_<versión>_amd64.deb`. Instalar con `sudo apt install ./Pixora_<versión>_amd64.deb`
(apt instala la dependencia `libvips42t64 | libvips42`).

### Windows (instalador `.exe`)

Requisitos (en Windows, desde una *Developer PowerShell for VS* con el entorno MSVC x64, para tener `dumpbin` y `lib`):

- Node (ver `.nvmrc`), Rust con toolchain MSVC y `pkg-config` (`choco install pkgconfiglite`)
- [Inno Setup 6](https://jrsoftware.org/isinfo.php) (`choco install innosetup`)
- libvips `vips-dev-w64-all` ([libvips/build-win64-mxe](https://github.com/libvips/build-win64-mxe/releases)) descomprimido, con su ruta en `VIPSHOME`

```powershell
$env:VIPSHOME = "C:\vips"
pwsh scripts/build-windows.ps1                          # o: npm run build:windows
pwsh scripts/build-windows.ps1 -SkipTests -SkipWebView2 # opciones
```

Qué hace: valida versiones, genera las import libs MSVC de libvips (`scripts/make-vips-implibs.ps1`), corre los tests, compila
`pixora.exe` con `tauri build --no-bundle` y llama a `scripts/package-windows.ps1` (copia las DLL de libvips junto al exe y compila
`installer/pixora.iss`). Salida: `out/Pixora-Setup-<versión>-x64.exe`.

### Microsoft Store (MSIX)

Producto reservado en Partner Center como **PixoraL**. Tras generar el instalador:

```powershell
pwsh scripts/package-msix.ps1 -Version 1.0.0     # o: build-windows.ps1 -Msix
```

Salida: `out/Pixora-<versión>-x64.msix` (requiere Windows SDK). Se sube **sin firmar** a Partner Center, que lo firma. El manifiesto
está en `installer/msix/AppxManifest.xml` y la identidad no debe cambiarse. Detalles en [`docs/release.md`](docs/release.md).

### Release automático (CI)

`.github/workflows/release.yml` compila en un runner de Windows el instalador (Inno Setup) y el paquete `.msix`:

- push a la rama `release` o ejecución manual → artefacto de Actions (sin release pública)
- tag `vX.Y.Z` → además crea el GitHub Release

Antes de publicar, subir la versión a la vez en `package.json`, `src-tauri/Cargo.toml` y `src-tauri/tauri.conf.json`
(`npm run check:versions` lo valida) y actualizar `CHANGELOG.md`. Detalle completo en [`docs/release.md`](docs/release.md).

## Estructura

Ver §5 de `PIXORA_SPEC.md`. Línea gráfica: [`docs/brand.md`](docs/brand.md); manual original en `brand/`.

## Logo

`logo.png` en la raíz es un recorte **provisional** del manual de marca. Sustituir por el original (1024×1024, transparencia) y ejecutar `scripts/generate-icons.sh`.
