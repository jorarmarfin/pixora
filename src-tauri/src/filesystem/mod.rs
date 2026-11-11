//! Validación de rutas y descubrimiento de archivos. La escritura segura
//! (temporales, colisiones) llega en la Fase 3.

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::domain::ImageFormat;

pub fn is_supported_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .and_then(ImageFormat::from_extension)
        .is_some()
}

/// Expande archivos y carpetas a una lista de rutas candidatas (únicas, ordenadas).
/// No sigue enlaces simbólicos (evita ciclos). Rutas inexistentes se ignoran.
/// Devuelve también los archivos explícitos con extensión no soportada.
pub struct Collected {
    pub images: Vec<PathBuf>,
    pub unsupported: Vec<PathBuf>,
}

pub fn collect_paths(inputs: &[PathBuf], recursive: bool) -> Collected {
    let mut images = Vec::new();
    let mut unsupported = Vec::new();
    for input in inputs {
        let Ok(meta) = std::fs::symlink_metadata(input) else {
            continue;
        };
        if meta.is_dir() {
            let walker = WalkDir::new(input)
                .follow_links(false)
                .max_depth(if recursive { usize::MAX } else { 1 });
            for entry in walker.into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() && is_supported_image(entry.path()) {
                    images.push(entry.into_path());
                }
            }
        } else if meta.is_file() || meta.file_type().is_symlink() {
            if is_supported_image(input) {
                images.push(input.clone());
            } else {
                unsupported.push(input.clone());
            }
        }
    }
    images.sort();
    images.dedup();
    Collected {
        images,
        unsupported,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn touch(p: &Path) {
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, b"x").unwrap();
    }

    #[test]
    fn recorre_carpeta_con_espacios_y_unicode() {
        let d = tempfile::tempdir().unwrap();
        touch(&d.path().join("fotos de verano/año ñandú.JPG"));
        touch(&d.path().join("fotos de verano/sub/日本.png"));
        touch(&d.path().join("fotos de verano/notas.txt"));
        let root = d.path().join("fotos de verano");

        let plano = collect_paths(std::slice::from_ref(&root), false);
        assert_eq!(plano.images.len(), 1);

        let rec = collect_paths(&[root], true);
        assert_eq!(rec.images.len(), 2);
    }

    #[test]
    fn archivo_explicito_no_soportado_se_reporta() {
        let d = tempfile::tempdir().unwrap();
        touch(&d.path().join("a.gif"));
        let c = collect_paths(
            &[d.path().join("a.gif"), d.path().join("no-existe.png")],
            false,
        );
        assert!(c.images.is_empty());
        assert_eq!(c.unsupported.len(), 1);
    }

    #[test]
    fn deduplica_rutas() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("a.png");
        touch(&f);
        let c = collect_paths(&[f.clone(), f, d.path().to_path_buf()], false);
        assert_eq!(c.images.len(), 1);
    }
}
