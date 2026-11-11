# Contrato IPC (Tauri)

| Comando | Entrada | Salida | Notas |
|---|---|---|---|
| `app_info` | — | `AppInfo` | versión y motor |
| `import_paths` | `paths: string[]`, `recursive: bool` | `ImportResult { items, skipped }` | lee solo cabeceras; no sigue symlinks; dedup por ruta canónica; archivos corruptos/no soportados van a `skipped` con `reason` |
| `get_thumbnail` | `path: string` | data URL JPEG (≤192 px) | respeta EXIF; el front limita a 3 peticiones concurrentes |

Drag & drop: evento nativo `onDragDropEvent` del webview (rutas reales). Diálogos: `tauri-plugin-dialog` (`dialog:allow-open`).
Tipos TS en `src/types/ipc.ts` (espejo manual de `src-tauri/src/domain`); valorar `specta` en Fase 4.
Lotes (`start_batch`, `cancel_batch`, `reveal_folder`, evento `batch://progress`): ver `docs/jobs.md`.
