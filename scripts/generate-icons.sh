#!/usr/bin/env bash
# Genera iconos derivados desde logo.png hacia src-tauri/icons/.
# Uso real (incl. .ico para Inno Setup) en la Fase 7.
set -Eeuo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
[[ -f logo.png ]] || { echo "Falta logo.png en la raíz." >&2; exit 1; }
npx tauri icon logo.png
rm -rf src-tauri/icons/android src-tauri/icons/ios
