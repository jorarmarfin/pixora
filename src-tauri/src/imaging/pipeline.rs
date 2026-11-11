//! Orquestación de un trabajo: validar → resolver destino → codificar a temporal → publicar.

use std::path::Path;
use std::time::Instant;

use super::engine::{EncodeRequest, ImageEngine};
use crate::domain::{ConflictPolicy, ImageJob, JobError, JobErrorCode, JobResult, JobStatus};
use crate::filesystem::{self, Resolution, TempFile};

pub struct Options {
    /// El usuario confirmó explícitamente que puede sobrescribir salidas existentes.
    pub overwrite_confirmed: bool,
}

pub fn run_job(engine: &dyn ImageEngine, job: &ImageJob, opts: &Options) -> JobResult {
    let started = Instant::now();
    let input = Path::new(&job.input_path);
    let input_bytes = std::fs::metadata(input).map(|m| m.len()).unwrap_or(0);
    let fail = |e: JobError| {
        JobResult::failed(
            &job.id,
            e,
            input_bytes,
            started.elapsed().as_millis() as u64,
        )
    };

    match run_inner(engine, job, opts, input_bytes) {
        Ok(Some(mut r)) => {
            r.duration_ms = started.elapsed().as_millis() as u64;
            r
        }
        Ok(None) => JobResult {
            id: job.id.clone(),
            status: JobStatus::Skipped,
            output_path: None,
            width: 0,
            height: 0,
            input_bytes,
            output_bytes: 0,
            duration_ms: started.elapsed().as_millis() as u64,
            error: None,
        },
        Err(e) => fail(e),
    }
}

fn run_inner(
    engine: &dyn ImageEngine,
    job: &ImageJob,
    opts: &Options,
    input_bytes: u64,
) -> Result<Option<JobResult>, JobError> {
    if job.conflict_policy == ConflictPolicy::Overwrite && !opts.overwrite_confirmed {
        return Err(JobError::new(JobErrorCode::OverwriteNotConfirmed));
    }
    let input = Path::new(&job.input_path);
    let input_canon = std::fs::canonicalize(input)
        .map_err(|e| filesystem::map_io(&e, JobErrorCode::InputNotFound))?;
    let out_dir = filesystem::validate_output_dir(Path::new(&job.output_directory))?;

    let stem = input_canon
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let resolution = filesystem::resolve_output(
        &out_dir,
        &stem,
        &job.name_suffix,
        job.output_format,
        job.conflict_policy,
        &input_canon,
    )?;
    let mut final_path = match resolution {
        Resolution::Skip => return Ok(None),
        Resolution::Path(p) => p,
    };

    // Temporal en el mismo directorio para poder publicar con rename atómico.
    let tmp = TempFile::new(&out_dir, job.output_format);
    let out = engine.encode(&EncodeRequest {
        input: &input_canon,
        output: tmp.path(),
        format: job.output_format,
        quality_mode: job.quality_mode,
        quality: job.quality_value,
        resize: job.resize,
        metadata: job.metadata_policy,
    })?;

    // Con lotes concurrentes otro worker puede ocupar el nombre entre resolver y publicar:
    // se vuelve a resolver (Rename elige el siguiente libre; Skip omite).
    let mut attempts = 0;
    let published = loop {
        if let Some(p) = tmp.publish(&final_path, job.conflict_policy)? {
            break p;
        }
        attempts += 1;
        if attempts > 100 {
            return Err(JobError::with_detail(
                JobErrorCode::Io,
                "el destino apareció durante el proceso",
            ));
        }
        match filesystem::resolve_output(
            &out_dir,
            &stem,
            &job.name_suffix,
            job.output_format,
            job.conflict_policy,
            &input_canon,
        )? {
            Resolution::Skip => return Ok(None),
            Resolution::Path(p) => final_path = p,
        }
    };
    let output_bytes = std::fs::metadata(&published).map(|m| m.len()).unwrap_or(0);
    Ok(Some(JobResult {
        id: job.id.clone(),
        status: JobStatus::Completed,
        output_path: Some(published.to_string_lossy().into_owned()),
        width: out.width,
        height: out.height,
        input_bytes,
        output_bytes,
        duration_ms: 0,
        error: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        FitMode, ImageFormat, JobErrorCode, MetadataPolicy, QualityMode, ResizeOptions,
    };
    use crate::imaging;
    use image::{ImageFormat as F, Rgb, RgbImage, Rgba, RgbaImage};
    use std::path::PathBuf;

    fn engine() -> &'static dyn ImageEngine {
        imaging::engine().expect("libvips")
    }

    fn job(input: &Path, out: &Path, format: ImageFormat) -> ImageJob {
        ImageJob {
            id: "t".into(),
            input_path: input.to_string_lossy().into_owned(),
            output_directory: out.to_string_lossy().into_owned(),
            output_format: format,
            quality_mode: QualityMode::Lossy,
            quality_value: 80,
            resize: ResizeOptions::None,
            metadata_policy: MetadataPolicy::Strip,
            name_suffix: "-pixora".into(),
            conflict_policy: ConflictPolicy::Rename,
        }
    }

    const NO_CONFIRM: Options = Options {
        overwrite_confirmed: false,
    };

    /// Imagen con ruido determinista (comprimible de forma distinta según calidad).
    fn noisy(w: u32, h: u32) -> RgbImage {
        RgbImage::from_fn(w, h, |x, y| {
            let v = x
                .wrapping_mul(2654435761)
                .wrapping_add(y.wrapping_mul(40503))
                >> 7;
            Rgb([
                (v & 255) as u8,
                ((v >> 3) & 255) as u8,
                ((v >> 6) & 255) as u8,
            ])
        })
    }

    fn save(img: &RgbImage, p: &Path, f: F) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        img.save_with_format(p, f).unwrap();
    }

    /// JPEG con etiqueta EXIF Orientation inyectada (APP1 mínimo).
    fn jpeg_with_orientation(w: u32, h: u32, orientation: u8) -> Vec<u8> {
        let mut buf = std::io::Cursor::new(Vec::new());
        RgbImage::from_pixel(w, h, Rgb([200, 30, 30]))
            .write_to(&mut buf, F::Jpeg)
            .unwrap();
        let jpeg = buf.into_inner();
        let tiff: Vec<u8> = [
            &b"II"[..],
            &[0x2A, 0, 8, 0, 0, 0],
            &[1, 0],
            &[0x12, 0x01, 3, 0, 1, 0, 0, 0, orientation, 0, 0, 0],
            &[0, 0, 0, 0],
        ]
        .concat();
        let mut app1 = b"Exif\0\0".to_vec();
        app1.extend(&tiff);
        let len = (app1.len() + 2) as u16;
        let mut out = vec![0xFF, 0xD8, 0xFF, 0xE1];
        out.extend(len.to_be_bytes());
        out.extend(app1);
        out.extend(&jpeg[2..]);
        out
    }

    fn contains(hay: &[u8], needle: &[u8]) -> bool {
        hay.windows(needle.len()).any(|w| w == needle)
    }

    fn leftovers(dir: &Path) -> Vec<PathBuf> {
        std::fs::read_dir(dir)
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.file_name().unwrap().to_string_lossy().contains(".tmp."))
            .collect()
    }

    #[test]
    fn convierte_png_a_webp_sin_tocar_el_origen() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("entrada/foto.png");
        save(&noisy(64, 48), &src, F::Png);
        let before = std::fs::read(&src).unwrap();
        let out = d.path().join("salida");
        std::fs::create_dir_all(&out).unwrap();

        let r = run_job(engine(), &job(&src, &out, ImageFormat::Webp), &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
        assert_eq!((r.width, r.height), (64, 48));
        let out_path = PathBuf::from(r.output_path.unwrap());
        assert_eq!(out_path.file_name().unwrap(), "foto-pixora.webp");
        let bytes = std::fs::read(&out_path).unwrap();
        assert_eq!(image::guess_format(&bytes).unwrap(), F::WebP);
        assert_eq!(r.output_bytes, bytes.len() as u64);
        assert_eq!(r.input_bytes, before.len() as u64);
        assert_eq!(std::fs::read(&src).unwrap(), before, "el origen cambió");
        assert!(leftovers(&out).is_empty());
    }

    #[test]
    fn formatos_de_salida_son_los_pedidos() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(32, 32), &src, F::Png);
        for (fmt, expected) in [
            (ImageFormat::Jpeg, F::Jpeg),
            (ImageFormat::Png, F::Png),
            (ImageFormat::Webp, F::WebP),
        ] {
            let out = d.path().join(format!("o_{expected:?}"));
            std::fs::create_dir_all(&out).unwrap();
            let r = run_job(engine(), &job(&src, &out, fmt), &NO_CONFIRM);
            assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
            let b = std::fs::read(r.output_path.unwrap()).unwrap();
            assert_eq!(image::guess_format(&b).unwrap(), expected);
        }
    }

    #[test]
    fn calidad_menor_pesa_menos_y_lossless_es_exacto() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("n.png");
        let img = noisy(128, 128);
        save(&img, &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let size_for = |q: u8, suffix: &str| {
            let mut j = job(&src, &out, ImageFormat::Jpeg);
            j.quality_value = q;
            j.name_suffix = suffix.into();
            run_job(engine(), &j, &NO_CONFIRM).output_bytes
        };
        assert!(size_for(10, "-q10") < size_for(95, "-q95"));

        let mut j = job(&src, &out, ImageFormat::Webp);
        j.quality_mode = QualityMode::Lossless;
        j.name_suffix = "-ll".into();
        let r = run_job(engine(), &j, &NO_CONFIRM);
        let decoded = image::open(r.output_path.unwrap()).unwrap().to_rgb8();
        assert_eq!(decoded, img, "WebP lossless debe ser idéntico");
    }

    #[test]
    fn redimensiona_por_ancho_y_cover() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(40, 20), &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();

        let mut j = job(&src, &out, ImageFormat::Png);
        j.resize = ResizeOptions::Width { px: 20 };
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!((r.width, r.height), (20, 10));
        let img = image::open(r.output_path.unwrap()).unwrap();
        assert_eq!((img.width(), img.height()), (20, 10));

        let mut j = job(&src, &out, ImageFormat::Png);
        j.name_suffix = "-cover".into();
        j.resize = ResizeOptions::Box {
            width: 16,
            height: 16,
            fit: FitMode::Cover,
        };
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!((r.width, r.height), (16, 16));
    }

    #[test]
    fn exif_se_corrige_antes_de_dimensionar() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("rot.jpg");
        std::fs::write(&src, jpeg_with_orientation(40, 20, 6)).unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();

        // Orientación 6 => se muestra de 20×40; luego ancho 10 => 10×20.
        let mut j = job(&src, &out, ImageFormat::Jpeg);
        j.resize = ResizeOptions::Width { px: 10 };
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
        assert_eq!((r.width, r.height), (10, 20));
    }

    #[test]
    fn metadatos_strip_elimina_exif_y_keep_all_lo_conserva() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("m.jpg");
        std::fs::write(&src, jpeg_with_orientation(40, 20, 1)).unwrap();
        assert!(contains(&std::fs::read(&src).unwrap(), b"Exif"));
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();

        let r = run_job(engine(), &job(&src, &out, ImageFormat::Jpeg), &NO_CONFIRM);
        assert!(!contains(
            &std::fs::read(r.output_path.unwrap()).unwrap(),
            b"Exif"
        ));

        let mut j = job(&src, &out, ImageFormat::Jpeg);
        j.metadata_policy = MetadataPolicy::KeepAll;
        j.name_suffix = "-keep".into();
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert!(contains(
            &std::fs::read(r.output_path.unwrap()).unwrap(),
            b"Exif"
        ));
    }

    #[test]
    fn transparencia_a_jpeg_se_aplana_sobre_blanco_y_png_la_conserva() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("t.png");
        RgbaImage::from_pixel(16, 16, Rgba([255, 0, 0, 0]))
            .save(&src)
            .unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();

        let r = run_job(engine(), &job(&src, &out, ImageFormat::Jpeg), &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
        let px = image::open(r.output_path.unwrap())
            .unwrap()
            .to_rgb8()
            .get_pixel(8, 8)
            .0;
        assert!(px.iter().all(|&c| c > 240), "esperaba blanco, fue {px:?}");

        let mut j = job(&src, &out, ImageFormat::Png);
        j.name_suffix = "-png".into();
        let r = run_job(engine(), &j, &NO_CONFIRM);
        let a = image::open(r.output_path.unwrap())
            .unwrap()
            .to_rgba8()
            .get_pixel(8, 8)
            .0[3];
        assert_eq!(a, 0, "PNG debe conservar el alfa");
    }

    #[test]
    fn colision_renombra_con_sufijo_incremental() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let names: Vec<String> = (0..3)
            .map(|_| {
                let r = run_job(engine(), &job(&src, &out, ImageFormat::Png), &NO_CONFIRM);
                PathBuf::from(r.output_path.unwrap())
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into()
            })
            .collect();
        assert_eq!(
            names,
            ["a-pixora.png", "a-pixora (1).png", "a-pixora (2).png"]
        );
        assert!(leftovers(&out).is_empty());
    }

    #[test]
    fn politica_omitir_no_escribe() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        run_job(engine(), &job(&src, &out, ImageFormat::Png), &NO_CONFIRM);
        let mut j = job(&src, &out, ImageFormat::Png);
        j.conflict_policy = ConflictPolicy::Skip;
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Skipped);
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), 1);
    }

    #[test]
    fn sobrescribir_exige_confirmacion_y_nunca_pisa_el_origen() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let before = std::fs::read(&src).unwrap();

        // Misma carpeta y sin sufijo => el destino sería el propio origen.
        let mut j = job(&src, d.path(), ImageFormat::Png);
        j.name_suffix = String::new();
        j.conflict_policy = ConflictPolicy::Overwrite;

        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!(r.error.unwrap().code, JobErrorCode::OverwriteNotConfirmed);

        let r = run_job(
            engine(),
            &j,
            &Options {
                overwrite_confirmed: true,
            },
        );
        assert_eq!(r.error.unwrap().code, JobErrorCode::SourceEqualsDestination);
        assert_eq!(std::fs::read(&src).unwrap(), before);
        assert!(leftovers(d.path()).is_empty());

        // Con otra salida existente, sí sobrescribe con confirmación.
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(out.join("a.png"), b"viejo").unwrap();
        let mut j = job(&src, &out, ImageFormat::Png);
        j.name_suffix = String::new();
        j.conflict_policy = ConflictPolicy::Overwrite;
        let r = run_job(
            engine(),
            &j,
            &Options {
                overwrite_confirmed: true,
            },
        );
        assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
        assert_ne!(std::fs::read(out.join("a.png")).unwrap(), b"viejo");
    }

    #[test]
    fn entrada_corrupta_falla_sin_dejar_temporales() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("rota.png");
        std::fs::write(&src, b"esto no es un png").unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let r = run_job(engine(), &job(&src, &out, ImageFormat::Webp), &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Failed);
        assert_eq!(r.error.unwrap().code, JobErrorCode::Corrupt);
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), 0);
    }

    #[test]
    fn errores_de_entrada_y_destino() {
        let d = tempfile::tempdir().unwrap();
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let r = run_job(
            engine(),
            &job(&d.path().join("no-existe.png"), &out, ImageFormat::Png),
            &NO_CONFIRM,
        );
        assert_eq!(r.error.unwrap().code, JobErrorCode::InputNotFound);

        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let r = run_job(
            engine(),
            &job(&src, &d.path().join("no-existe"), ImageFormat::Png),
            &NO_CONFIRM,
        );
        assert_eq!(r.error.unwrap().code, JobErrorCode::InvalidOutputDir);
        let r = run_job(
            engine(),
            &job(&src, Path::new(""), ImageFormat::Png),
            &NO_CONFIRM,
        );
        assert_eq!(r.error.unwrap().code, JobErrorCode::InvalidOutputDir);
    }

    /// Regresión: matriz formato × redimensionado con dimensiones y formato exactos.
    #[test]
    fn matriz_de_regresion_de_salidas() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(200, 100), &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let cases: [(ResizeOptions, (u32, u32)); 6] = [
            (ResizeOptions::None, (200, 100)),
            (ResizeOptions::Width { px: 100 }, (100, 50)),
            (ResizeOptions::Height { px: 25 }, (50, 25)),
            (ResizeOptions::Percent { value: 10.0 }, (20, 10)),
            (
                ResizeOptions::Box {
                    width: 60,
                    height: 60,
                    fit: FitMode::Contain,
                },
                (60, 30),
            ),
            (
                ResizeOptions::Box {
                    width: 60,
                    height: 60,
                    fit: FitMode::Cover,
                },
                (60, 60),
            ),
        ];
        for (fmt, expected) in [
            (ImageFormat::Jpeg, F::Jpeg),
            (ImageFormat::Png, F::Png),
            (ImageFormat::Webp, F::WebP),
        ] {
            for (i, (resize, dims)) in cases.iter().enumerate() {
                let mut j = job(&src, &out, fmt);
                j.resize = *resize;
                j.name_suffix = format!("-{expected:?}-{i}");
                let r = run_job(engine(), &j, &NO_CONFIRM);
                assert_eq!(
                    r.status,
                    JobStatus::Completed,
                    "{fmt:?} {resize:?}: {:?}",
                    r.error
                );
                assert_eq!((r.width, r.height), *dims, "{fmt:?} {resize:?}");
                let bytes = std::fs::read(r.output_path.unwrap()).unwrap();
                assert_eq!(image::guess_format(&bytes).unwrap(), expected);
                let img = image::load_from_memory(&bytes).unwrap();
                assert_eq!((img.width(), img.height()), *dims);
            }
        }
        assert!(leftovers(&out).is_empty());
    }

    /// PNG→PNG sin redimensionar conserva los píxeles exactos.
    #[test]
    fn png_a_png_es_exacto_pixel_a_pixel() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        let img = noisy(37, 29);
        save(&img, &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let r = run_job(engine(), &job(&src, &out, ImageFormat::Png), &NO_CONFIRM);
        assert_eq!(image::open(r.output_path.unwrap()).unwrap().to_rgb8(), img);
    }

    #[test]
    fn opciones_invalidas_se_reportan() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let out = d.path().join("o");
        std::fs::create_dir_all(&out).unwrap();
        let mut j = job(&src, &out, ImageFormat::Png);
        j.resize = ResizeOptions::Width { px: 0 };
        let r = run_job(engine(), &j, &NO_CONFIRM);
        assert_eq!(r.error.unwrap().code, JobErrorCode::InvalidOptions);
        assert_eq!(std::fs::read_dir(&out).unwrap().count(), 0);
    }

    #[test]
    fn rutas_con_espacios_y_unicode() {
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("mis fotos/año ñandú 日本.png");
        save(&noisy(8, 8), &src, F::Png);
        let out = d.path().join("salida ü/sub carpeta");
        std::fs::create_dir_all(&out).unwrap();
        let r = run_job(engine(), &job(&src, &out, ImageFormat::Webp), &NO_CONFIRM);
        assert_eq!(r.status, JobStatus::Completed, "{:?}", r.error);
        assert!(PathBuf::from(r.output_path.unwrap()).ends_with("año ñandú 日本-pixora.webp"));
    }

    #[cfg(unix)]
    #[test]
    fn sin_permisos_de_escritura() {
        use std::os::unix::fs::PermissionsExt;
        let d = tempfile::tempdir().unwrap();
        let src = d.path().join("a.png");
        save(&noisy(8, 8), &src, F::Png);
        let out = d.path().join("ro");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::set_permissions(&out, std::fs::Permissions::from_mode(0o555)).unwrap();
        let writable = std::fs::File::create(out.join("probe")).is_ok();
        if !writable {
            let r = run_job(engine(), &job(&src, &out, ImageFormat::Png), &NO_CONFIRM);
            assert_ne!(r.status, JobStatus::Completed);
            assert!(matches!(
                r.error.unwrap().code,
                JobErrorCode::PermissionDenied | JobErrorCode::Engine
            ));
        }
        std::fs::set_permissions(&out, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}
