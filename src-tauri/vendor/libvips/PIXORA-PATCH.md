# Parche local de libvips 1.6.1 (olxgroup-oss/libvips-rust-bindings, MIT)

Cambio único en `src/bindings.rs`: `size_t`, `gsize`, `gssize`, `gint64`, `guint64`, `gintptr`, `guintptr`,
`ssize_t` y `time_t` pasan de `c_long`/`c_ulong` a `u64`/`i64`. Los bindings originales se generaron en
Linux (LP64); en Windows x64 (LLP64) `c_long` mide 32 bits y el crate no compilaba (E0308). Solo x86_64.
Se aplica con `[patch.crates-io]` en `src-tauri/Cargo.toml`.
