# Lotes: concurrencia y estados

## Estados de un trabajo
`queued → processing → completed | failed | skipped`, o `queued → cancelled` (nunca llegó a iniciar).
El front marca `processing` al recibir `started`; el backend solo emite estados terminales en `finished`.

## Scheduler (`src-tauri/src/jobs/mod.rs`)
- `run_batch` lanza N workers (`std::thread::scope`, N = 1–8, por defecto 3) que toman el siguiente
  índice con un `AtomicUsize`. Cada worker ejecuta `pipeline::run_job` completo:
  validar → resolver destino → codificar a temporal (mismo directorio) → publicar (hard_link / rename).
- Los resultados se devuelven en el orden de entrada. Un error o pánico en un archivo se convierte en
  `failed` y no aborta el lote.
- **Colisiones concurrentes**: si otro worker ocupa el nombre entre resolver y publicar, el pipeline
  vuelve a resolver (`Rename` → siguiente `(n)` libre; `Skip` → omitido). Nunca se pisa una salida.

## Cancelación
- `cancel_batch` activa un `AtomicBool`. Se comprueba **antes de iniciar** cada trabajo: los pendientes
  quedan `cancelled`; los activos terminan (libvips no es interrumpible → granularidad por archivo).
- Los temporales se eliminan en `Drop` (`TempFile`), por lo que no quedan `.tmp` tras error o cancelación.

## Comandos y eventos
| | |
|---|---|
| `start_batch { jobs, overwrite_confirmed, concurrency? }` | bloquea hasta terminar; devuelve `{ summary, results }`. Solo un lote a la vez |
| `cancel_batch` | `bool` (había lote) |
| `reveal_folder { path }` | abre una carpeta existente en el explorador |
| evento `batch://progress` | `{ total, done, started[], finished[] }`, agrupado cada ≥100 ms (+ flush final) |

`summary`: `total, completed, failed, skipped, cancelled, input_bytes, output_bytes, saved_bytes, duration_ms, was_cancelled`
(bytes solo de completados; `saved_bytes` negativo si la salida pesa más).
