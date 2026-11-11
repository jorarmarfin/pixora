#!/usr/bin/env bash
# Comprueba el entorno de desarrollo de Pixora. No instala nada.
set -u
cd "$(dirname "${BASH_SOURCE[0]}")/.."
fail=0
check() { # nombre, comando de versión
  if command -v "$1" >/dev/null 2>&1; then printf "ok    %-12s %s\n" "$1" "$($@ --version 2>&1 | head -1)"; else printf "FALTA %s\n" "$1"; fail=1; fi
}
for c in node npm cargo rustc pkg-config; do check "$c"; done
for lib in webkit2gtk-4.1 vips; do
  if pkg-config --exists "$lib" 2>/dev/null; then printf "ok    %-12s %s\n" "$lib" "$(pkg-config --modversion "$lib")"; else printf "FALTA %s%s\n" "$lib" "$([[ $lib == vips ]] && echo ' (requerido desde la Fase 3)')"; [[ $lib == vips ]] || fail=1; fi
done
exit $fail
