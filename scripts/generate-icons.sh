#!/usr/bin/env bash
# Genera los iconos derivados desde logo.png (1024×1024, transparencia):
#  - src-tauri/icons/* (png, icns, ico) para Tauri
#  - el .ico lo usa también Inno Setup (installer/pixora.iss → SetupIconFile)
set -Eeuo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
[[ -f logo.png ]] || { echo "Falta logo.png en la raíz." >&2; exit 1; }
npx tauri icon logo.png
rm -rf src-tauri/icons/android src-tauri/icons/ios
[[ -f src-tauri/icons/icon.ico ]] || { echo "No se generó icon.ico" >&2; exit 1; }
echo "Iconos generados en src-tauri/icons/"
