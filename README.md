# Pixora

Aplicación de escritorio **offline-first** para convertir, comprimir, optimizar y transformar imágenes. Spec: [`PIXORA_SPEC.md`](PIXORA_SPEC.md). Plan: [`.claude/spec-plan.md`](.claude/spec-plan.md).

## Requisitos (Ubuntu)

- Node.js LTS (ver `.nvmrc`) y npm
- Rust estable (`rustup`), `pkg-config`
- Tauri 2: `sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev`
- libvips (desde la Fase 3): `sudo apt install libvips-dev`

Comprobar entorno: `scripts/check-env.sh`

## Desarrollo

```bash
./run.sh          # modo desarrollo (tauri dev); no genera instaladores
npm test          # Vitest
npm run lint && npm run typecheck
cd src-tauri && cargo test && cargo clippy -- -D warnings
```

## Estructura

Ver §5 de `PIXORA_SPEC.md`. Línea gráfica: [`docs/brand.md`](docs/brand.md); manual original en `brand/`.

## Logo

`logo.png` en la raíz es un recorte **provisional** del manual de marca. Sustituir por el original (1024×1024, transparencia) y ejecutar `scripts/generate-icons.sh`.
