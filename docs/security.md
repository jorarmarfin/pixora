# Revisión de seguridad (Fase 6)

| Área | Estado |
|---|---|
| Capabilities Tauri | mínimo privilegio: `core:default` + `dialog:allow-open`; sin shell, fs ni http |
| CSP | restrictivo (`default-src 'self'`, `img-src 'self' data:`, `connect-src ipc:`); `devCsp` añade el HMR de Vite |
| Shell | solo `reveal_folder` lanza `xdg-open`/`explorer`/`open`, con la ruta como **argumento** (sin shell) y solo si es un directorio existente |
| Rutas | canonicalizadas; destino validado; nunca se escribe sobre el origen; sobrescritura exige confirmación del usuario |
| Escritura | temporal en el mismo directorio + `hard_link`/`rename` (sin pisar por carrera); `Drop` limpia temporales |
| Bombas de descompresión | 250 MP en el motor (se lee la cabecera antes de decodificar); 64 MP en miniaturas |
| Concurrencia | 1–8 workers acotados; un lote a la vez |
| Config | JSON validado y saneado al cargar; corrupto → `.bak` + defaults |
| Red / telemetría | ninguna |
