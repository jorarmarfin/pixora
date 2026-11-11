use std::io::Cursor;
use std::path::{Path, PathBuf};

use base64::Engine;
use image::{DynamicImage, ImageDecoder, ImageReader};

use crate::domain::{ImageFormat, ImageItem, ImportResult, SkipReason, SkippedFile};
use crate::filesystem::collect_paths;

const THUMB_SIZE: u32 = 96;

fn file_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Lee metadatos solo de la cabecera (no decodifica píxeles).
pub fn read_item(path: &Path) -> Result<ImageItem, SkipReason> {
    let canonical = std::fs::canonicalize(path).map_err(|_| SkipReason::Unreadable)?;
    let size_bytes = std::fs::metadata(&canonical)
        .map_err(|_| SkipReason::Unreadable)?
        .len();
    let format = canonical
        .extension()
        .and_then(|e| e.to_str())
        .and_then(ImageFormat::from_extension)
        .ok_or(SkipReason::Unsupported)?;
    let dim = imagesize::size(&canonical).map_err(|e| match e {
        // Un archivo truncado/basura da EOF: es corrupto, no ilegible.
        imagesize::ImageError::IoError(e) if e.kind() != std::io::ErrorKind::UnexpectedEof => {
            SkipReason::Unreadable
        }
        _ => SkipReason::Corrupt,
    })?;
    if dim.width == 0 || dim.height == 0 {
        return Err(SkipReason::Corrupt);
    }
    let path_str = canonical.to_string_lossy().into_owned();
    Ok(ImageItem {
        id: path_str.clone(),
        path: path_str,
        name: file_name(&canonical),
        width: dim.width as u32,
        height: dim.height as u32,
        format,
        size_bytes,
    })
}

pub fn import_blocking(inputs: Vec<PathBuf>, recursive: bool) -> ImportResult {
    let collected = collect_paths(&inputs, recursive);
    let mut result = ImportResult::default();
    for p in collected.unsupported {
        result.skipped.push(SkippedFile {
            name: file_name(&p),
            reason: SkipReason::Unsupported,
        });
    }
    let mut seen = std::collections::HashSet::new();
    for p in collected.images {
        match read_item(&p) {
            Ok(item) => {
                if seen.insert(item.id.clone()) {
                    result.items.push(item);
                }
            }
            Err(reason) => result.skipped.push(SkippedFile {
                name: file_name(&p),
                reason,
            }),
        }
    }
    result
}

#[tauri::command]
pub async fn import_paths(paths: Vec<String>, recursive: bool) -> Result<ImportResult, String> {
    let inputs = paths.into_iter().map(PathBuf::from).collect();
    tauri::async_runtime::spawn_blocking(move || import_blocking(inputs, recursive))
        .await
        .map_err(|e| e.to_string())
}

/// Tope de píxeles para decodificar una miniatura completa (64 MP ≈ 192 MB en RGB).
/// Evita que una "bomba de descompresión" agote la memoria; la fila queda sin miniatura.
pub const MAX_THUMB_PIXELS: u64 = 64_000_000;

/// Miniatura JPEG en data URL, respetando la orientación EXIF.
pub fn make_thumbnail(path: &Path) -> Result<String, String> {
    make_thumbnail_limited(path, MAX_THUMB_PIXELS)
}

pub fn make_thumbnail_limited(path: &Path, max_pixels: u64) -> Result<String, String> {
    let reader = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .with_guessed_format()
        .map_err(|e| e.to_string())?;
    let mut decoder = reader.into_decoder().map_err(|e| e.to_string())?;
    let (w, h) = decoder.dimensions();
    if u64::from(w) * u64::from(h) > max_pixels {
        return Err("imagen demasiado grande para miniatura".into());
    }
    let orientation = decoder.orientation().map_err(|e| e.to_string())?;
    let mut img = DynamicImage::from_decoder(decoder).map_err(|e| e.to_string())?;
    img.apply_orientation(orientation);
    let thumb = img.thumbnail(THUMB_SIZE * 2, THUMB_SIZE * 2).to_rgb8();
    let mut buf = Cursor::new(Vec::new());
    thumb
        .write_to(&mut buf, image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(buf.into_inner());
    Ok(format!("data:image/jpeg;base64,{b64}"))
}

#[tauri::command]
pub async fn get_thumbnail(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || make_thumbnail(Path::new(&path)))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageFormat as F, RgbImage};

    fn write_img(p: &Path, w: u32, h: u32, f: F) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        RgbImage::new(w, h).save_with_format(p, f).unwrap();
    }

    #[test]
    fn importa_metadatos_de_cabecera() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("mi foto ñ.png");
        write_img(&p, 40, 30, F::Png);
        let r = import_blocking(vec![p.clone()], false);
        assert_eq!(r.items.len(), 1);
        let it = &r.items[0];
        assert_eq!((it.width, it.height), (40, 30));
        assert_eq!(it.format, ImageFormat::Png);
        assert_eq!(it.name, "mi foto ñ.png");
        assert!(it.size_bytes > 0);
    }

    #[test]
    fn corrupto_y_no_soportado_no_abortan() {
        let d = tempfile::tempdir().unwrap();
        write_img(&d.path().join("ok.jpg"), 10, 10, F::Jpeg);
        std::fs::write(d.path().join("roto.png"), b"no soy png").unwrap();
        std::fs::write(d.path().join("anim.gif"), b"GIF89a").unwrap();
        let r = import_blocking(
            vec![d.path().to_path_buf(), d.path().join("anim.gif")],
            false,
        );
        assert_eq!(r.items.len(), 1);
        assert!(r.skipped.iter().any(|s| s.reason == SkipReason::Corrupt));
        assert!(
            r.skipped
                .iter()
                .any(|s| s.reason == SkipReason::Unsupported)
        );
    }

    #[test]
    fn lote_de_100_imagenes() {
        let d = tempfile::tempdir().unwrap();
        for i in 0..100 {
            write_img(&d.path().join(format!("img {i}.png")), 8, 8, F::Png);
        }
        let t = std::time::Instant::now();
        let r = import_blocking(vec![d.path().to_path_buf()], false);
        assert_eq!(r.items.len(), 100);
        assert!(t.elapsed().as_secs() < 5);
    }

    #[test]
    fn miniatura_es_data_url_jpeg() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("a.png");
        write_img(&p, 500, 300, F::Png);
        let url = make_thumbnail(&p).unwrap();
        assert!(url.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn miniatura_respeta_el_limite_de_pixeles() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("g.png");
        write_img(&p, 100, 100, F::Png);
        assert!(make_thumbnail_limited(&p, 10_000).is_ok());
        let e = make_thumbnail_limited(&p, 9_999).unwrap_err();
        assert!(e.contains("demasiado grande"));
    }

    #[test]
    fn miniatura_de_corrupto_da_error() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("x.png");
        std::fs::write(&p, b"basura").unwrap();
        assert!(make_thumbnail(&p).is_err());
    }
}
