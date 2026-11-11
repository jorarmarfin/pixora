#!/usr/bin/env bash
# Pixora — arranque en modo desarrollo (Linux). No genera instaladores.
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

if ! pkg-config --exists webkit2gtk-4.1; then
  echo "Pixora: falta webkit2gtk-4.1 (libwebkit2gtk-4.1-dev). Revisa README.md." >&2
  exit 1
fi

if ! pkg-config --exists vips; then
  echo "Pixora: falta libvips (motor de imagen). Instala: sudo apt install libvips-dev" >&2
  exit 1
fi

if [[ ! -d node_modules ]]; then
  npm ci
fi

exec npm run tauri dev -- "$@"
