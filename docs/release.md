# Guía de release

## Estrategia de libvips
| Plataforma | Cómo llega libvips |
|---|---|
| Linux `.deb` | dependencia dinámica `libvips42t64 | libvips42` (apt la instala) |
| Windows | binarios oficiales `vips-dev-w64-all` (libvips/build-win64-mxe); **todas las DLL** de `bin\` se copian junto a `pixora.exe` → arranca sin libvips preinstalado |
| Linux AppImage | no incluido en este MVP (habría que empaquetar libvips + WebKitGTK; probar en varias distros) |

Licencias: ver `docs/third-party.md` (libvips LGPL-2.1+, enlace dinámico).

## Flujo de versiones
1. Subir la versión a la vez en `package.json`, `src-tauri/Cargo.toml` y `src-tauri/tauri.conf.json`
   (`npm run check:versions` lo valida; el workflow también).
2. Actualizar `CHANGELOG.md`. Merge a `main`; promover a la rama `release`.
3. Push a `release` → `release.yml` construye Linux (`.deb`) y Windows (`Pixora-Setup-X.Y.Z-x64.exe`) y los deja como
   **artefactos de Actions** (sin release pública). Probarlos en máquinas limpias.
4. Crear el tag sobre el commit validado: `git tag vX.Y.Z && git push origin vX.Y.Z` → job `publish` crea el GitHub Release
   (único job con `contents: write`).

## Build local
- Linux: `npm run tauri build -- --bundles deb` → `src-tauri/target/release/bundle/deb/`.
- Windows (PowerShell, con libvips en `$env:VIPSHOME`, pkg-config e Inno Setup 6):
  `npm run tauri build -- --no-bundle; pwsh scripts/package-windows.ps1 -Version X.Y.Z` → `out/Pixora-Setup-X.Y.Z-x64.exe`.
- Iconos: sustituir `logo.png` (1024×1024, transparencia) y ejecutar `scripts/generate-icons.sh`.

## Instalador de Windows (`installer/pixora.iss`)
x64, instalación por usuario (sin admin; el usuario puede elegir todos los usuarios), español, acceso directo en menú
Inicio (y escritorio opcional), desinstalador, instala WebView2 solo si falta. `AppId` es un GUID fijo: **no cambiarlo**.

## Firma de código y SmartScreen
Sin certificado de firma, Windows SmartScreen mostrará "Windows protegió su PC" al ejecutar el instalador
(*Más información → Ejecutar de todas formas*). Recomendado antes de distribuir ampliamente: certificado de firma de código
(o Azure Trusted Signing) y firmar `pixora.exe` y el instalador con `signtool` antes/después de ISCC.

## Verificación manual antes de publicar (checklist)
- [ ] Linux limpio: `sudo apt install ./Pixora_X.Y.Z_amd64.deb` → abre → convierte → `sudo apt remove pixora`.
- [ ] Windows limpio (sin libvips): instalar → abrir → convertir → desinstalar (verificar que no quedan archivos).
- [ ] Sin conexión: todo el flujo funciona.

## Windows: parche del crate libvips (importante)
El crate `libvips 1.6.1` trae bindings generados en Linux (`size_t = c_ulong`); en Windows x64 esos tipos miden 32 bits y
no compila (E0308). Se usa una copia parcheada en `src-tauri/vendor/libvips` (ver `PIXORA-PATCH.md`) mediante
`[patch.crates-io]`. Además su `build.rs` enlaza `vips`, `glib-2.0` y `gobject-2.0`: `scripts/make-vips-implibs.ps1`
genera esas `.lib` MSVC desde las DLL de MXE. Comprobado desde Linux: `cargo check --target x86_64-pc-windows-msvc`;
el enlazado real en Windows **solo se prueba en el runner**.
