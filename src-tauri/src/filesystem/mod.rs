//! Validación de rutas y descubrimiento de archivos. La escritura segura
//! (temporales, colisiones) llega en la Fase 3.

use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::domain::{ConflictPolicy, ImageFormat, JobError, JobErrorCode};

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

// ---- Escritura segura ----

pub fn extension_of(f: ImageFormat) -> &'static str {
    match f {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        ImageFormat::Webp => "webp",
    }
}

/// Traduce un error de E/S a un error estructurado (sin rutas).
pub fn map_io(e: &io::Error, not_found: JobErrorCode) -> JobError {
    match e.kind() {
        io::ErrorKind::NotFound => JobError::new(not_found),
        io::ErrorKind::PermissionDenied => JobError::new(JobErrorCode::PermissionDenied),
        io::ErrorKind::StorageFull | io::ErrorKind::QuotaExceeded => {
            JobError::new(JobErrorCode::DiskFull)
        }
        _ => JobError::with_detail(JobErrorCode::Io, format!("{:?}", e.kind())),
    }
}

/// La carpeta de salida debe existir y ser directorio; se devuelve canónica.
pub fn validate_output_dir(dir: &Path) -> Result<PathBuf, JobError> {
    if dir.as_os_str().is_empty() {
        return Err(JobError::new(JobErrorCode::InvalidOutputDir));
    }
    let canon =
        std::fs::canonicalize(dir).map_err(|e| map_io(&e, JobErrorCode::InvalidOutputDir))?;
    if !canon.is_dir() {
        return Err(JobError::new(JobErrorCode::InvalidOutputDir));
    }
    Ok(canon)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Resolution {
    Path(PathBuf),
    Skip,
}

/// Nombre sin separadores ni caracteres problemáticos en el sufijo.
fn sanitize_suffix(s: &str) -> String {
    s.chars()
        .filter(|c| {
            !matches!(
                c,
                '/' | '\\' | '\0' | ':' | '*' | '?' | '"' | '<' | '>' | '|'
            )
        })
        .collect()
}

/// Decide la ruta final. Reglas: nunca se escribe sobre el archivo de origen;
/// `Rename` busca `nombre (1).ext`, `(2)`…; `Skip` omite si existe; `Overwrite` reemplaza
/// (requiere confirmación aguas arriba) salvo que sea el propio origen.
pub fn resolve_output(
    out_dir: &Path,
    stem: &str,
    suffix: &str,
    format: ImageFormat,
    policy: ConflictPolicy,
    input_canon: &Path,
) -> Result<Resolution, JobError> {
    let ext = extension_of(format);
    let base = format!("{stem}{}", sanitize_suffix(suffix));
    let candidate = out_dir.join(format!("{base}.{ext}"));
    let is_source = |p: &Path| {
        std::fs::canonicalize(p)
            .map(|c| c == input_canon)
            .unwrap_or(false)
    };

    if !candidate.exists() {
        return Ok(Resolution::Path(candidate));
    }
    match policy {
        ConflictPolicy::Skip => Ok(Resolution::Skip),
        ConflictPolicy::Overwrite => {
            if is_source(&candidate) {
                Err(JobError::new(JobErrorCode::SourceEqualsDestination))
            } else {
                Ok(Resolution::Path(candidate))
            }
        }
        ConflictPolicy::Rename => {
            for n in 1..10_000u32 {
                let c = out_dir.join(format!("{base} ({n}).{ext}"));
                if !c.exists() {
                    return Ok(Resolution::Path(c));
                }
            }
            Err(JobError::with_detail(
                JobErrorCode::Io,
                "demasiadas colisiones",
            ))
        }
    }
}

/// Archivo temporal en el directorio de destino. Se elimina si no se publica
/// (error, cancelación o pánico).
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn new(dir: &Path, format: ImageFormat) -> Self {
        let id = TMP_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // La extensión real es necesaria: libvips elige el codificador por ella.
        let name = format!(
            ".pixora-{}-{id}.tmp.{}",
            std::process::id(),
            extension_of(format)
        );
        Self {
            path: dir.join(name),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Publica el temporal en `final_path`. Con `Rename`/`Skip` no pisa nada
    /// (hard_link falla si existe); con `Overwrite` usa rename atómico.
    /// Devuelve `Ok(None)` si el destino ya existe (otro hilo lo ocupó): el llamador decide.
    pub fn publish(
        &self,
        final_path: &Path,
        policy: ConflictPolicy,
    ) -> Result<Option<PathBuf>, JobError> {
        let res = match policy {
            ConflictPolicy::Overwrite => std::fs::rename(&self.path, final_path),
            _ => std::fs::hard_link(&self.path, final_path),
        };
        match res {
            // El temporal se elimina en Drop (con rename ya no existe).
            Ok(()) => Ok(Some(final_path.to_path_buf())),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(e) => Err(map_io(&e, JobErrorCode::InvalidOutputDir)),
        }
    }
}

static TMP_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
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
