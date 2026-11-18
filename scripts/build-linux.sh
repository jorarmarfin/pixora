#!/usr/bin/env bash
# Pixora — compilación de release para Linux: genera el .deb.
# Uso: scripts/build-linux.sh [--skip-tests]
# Salida: src-tauri/target/release/bundle/deb/Pixora_<versión>_amd64.deb
set -Eeuo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

skip_tests=0
[[ "${1:-}" == "--skip-tests" ]] && skip_tests=1

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "Pixora: build-linux.sh solo corre en Linux." >&2; exit 1
fi

scripts/check-env.sh
if ! pkg-config --exists vips; then
  echo "Pixora: falta libvips. Instala: sudo apt install libvips-dev" >&2; exit 1
fi

node scripts/check-versions.mjs
[[ -d node_modules ]] || npm ci

if (( ! skip_tests )); then
  npm test
  (cd src-tauri && cargo test)
fi

npm run tauri build -- --bundles deb

echo
echo "OK:"
ls -1 src-tauri/target/release/bundle/deb/*.deb
