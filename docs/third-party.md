# Componentes de terceros

| Componente | Licencia | Cómo se distribuye |
|---|---|---|
| libvips | LGPL-2.1-or-later | enlace **dinámico** (`libvips-42.dll` en Windows, `libvips42` como dependencia del `.deb`); sustituible por el usuario |
| Dependencias de libvips (glib, libpng, libjpeg-turbo, libwebp, lcms2, …) | LGPL/BSD/zlib/MIT, según paquete | DLL incluidas en el instalador de Windows con sus licencias |
| Tauri / WebView2 / WebKitGTK | MIT/Apache-2.0 · runtime de Microsoft · LGPL | WebView2 se instala con su bootstrapper si falta |
| Crates Rust y paquetes npm | MIT/Apache-2.0 mayoritariamente | compilados en el binario |
| Fuente Outfit | SIL OFL 1.1 | empaquetada localmente (`@fontsource-variable/outfit`) |

El binding Rust `libvips` (crate) se usa con enlace dinámico; el adaptador está aislado tras el trait `ImageEngine`
(`src-tauri/src/imaging/`) para poder sustituir el motor sin reescribir funcionalidades.
