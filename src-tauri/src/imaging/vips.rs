//! Adaptador libvips. Único módulo que conoce el crate `libvips`.

use libvips::ops::{self, BandFormat, Interpretation};
use libvips::{VipsApp, VipsImage};

use super::engine::{ALPHA_BACKGROUND, EncodeOutput, EncodeRequest, ImageEngine, MAX_INPUT_PIXELS};
use super::geometry::target_size;
use crate::domain::{ImageFormat, JobError, JobErrorCode, MetadataPolicy, QualityMode};

pub struct VipsEngine;

impl VipsEngine {
    /// Inicializa libvips. `VipsApp` debe vivir todo el proceso: se "olvida" a propósito
    /// (libvips libera sus recursos al salir) y solo se llama una vez (ver `imaging::engine`).
    pub fn init() -> Result<Self, String> {
        let app = VipsApp::new("Pixora", false).map_err(|e| format!("libvips: {e}"))?;
        app.concurrency_set(2);
        std::mem::forget(app);
        Ok(Self)
    }
}

/// Con este binding (libvips <= 8.14 API) la conservación es todo o nada:
/// `Strip` elimina metadatos; `KeepIcc` y `KeepAll` los conservan. El filtrado fino
/// (solo ICC) queda para cuando se actualice el binding (post-MVP, spec §3.2).
fn strip_of(policy: MetadataPolicy) -> bool {
    policy == MetadataPolicy::Strip
}

fn scale_to(img: &VipsImage, w: i32, h: i32) -> Result<VipsImage, JobError> {
    let opts = ops::ResizeOptions {
        vscale: h as f64 / img.get_height() as f64,
        ..Default::default()
    };
    let out = ops::resize_with_opts(img, w as f64 / img.get_width() as f64, &opts)
        .map_err(engine_err("resize"))?;
    // Redondeos de escala pueden dar ±1 px: forzar medidas exactas pedidas.
    if (out.get_width(), out.get_height()) == (w, h) {
        return Ok(out);
    }
    let force = ops::ResizeOptions {
        vscale: h as f64 / out.get_height() as f64,
        ..Default::default()
    };
    ops::resize_with_opts(&out, w as f64 / out.get_width() as f64, &force)
        .map_err(engine_err("resize"))
}

fn engine_err(stage: &str) -> impl Fn(libvips::error::Error) -> JobError + '_ {
    move |e| JobError::with_detail(JobErrorCode::Engine, format!("{stage}: {e}"))
}

impl ImageEngine for VipsEngine {
    fn name(&self) -> &'static str {
        "libvips"
    }

    fn encode(&self, req: &EncodeRequest) -> Result<EncodeOutput, JobError> {
        let input = req.input.to_str().ok_or_else(|| {
            JobError::with_detail(JobErrorCode::InvalidOptions, "ruta de entrada no es UTF-8")
        })?;
        let output = req.output.to_str().ok_or_else(|| {
            JobError::with_detail(JobErrorCode::InvalidOptions, "ruta de salida no es UTF-8")
        })?;

        // Decodificación diferida (se lee al ejecutar el pipeline).
        let mut img =
            VipsImage::new_from_file(input).map_err(|_| JobError::new(JobErrorCode::Corrupt))?;

        if (img.get_width().max(0) as u64) * (img.get_height().max(0) as u64) > MAX_INPUT_PIXELS {
            return Err(JobError::new(JobErrorCode::TooLarge));
        }

        // 1) Orientación EXIF antes de dimensionar.
        img = ops::autorot(&img).map_err(engine_err("autorot"))?;

        // 2) Sin conservar perfil: llevar a sRGB para no alterar los colores al descartarlo.
        if req.metadata == MetadataPolicy::Strip {
            let opts = ops::IccTransformOptions {
                embedded: true,
                ..Default::default()
            };
            if let Ok(t) = ops::icc_transform_with_opts(&img, "srgb", &opts) {
                img = t;
            }
        }
        if matches!(img.get_interpretation(), Ok(Interpretation::Cmyk)) {
            img =
                ops::colourspace(&img, Interpretation::Srgb).map_err(engine_err("colourspace"))?;
        }

        // 3) Redimensionado (resize de libvips; Lanczos3 por defecto).
        let (w, h) = (img.get_width(), img.get_height());
        let target = target_size(w as u32, h as u32, &req.resize)?;
        let (tw, th) = (target.width as i32, target.height as i32);
        if target.crop {
            // Cover: llenar la caja y recortar al centro.
            let f = (tw as f64 / w as f64).max(th as f64 / h as f64);
            let rw = ((w as f64 * f).round() as i32).max(tw);
            let rh = ((h as f64 * f).round() as i32).max(th);
            img = scale_to(&img, rw, rh)?;
            img = ops::extract_area(&img, (rw - tw) / 2, (rh - th) / 2, tw, th)
                .map_err(engine_err("crop"))?;
        } else if (tw, th) != (w, h) {
            img = scale_to(&img, tw, th)?;
        }

        // 4) JPEG no admite transparencia: aplanar sobre fondo.
        if req.format == ImageFormat::Jpeg && img.image_hasalpha() {
            let opts = ops::FlattenOptions {
                background: ALPHA_BACKGROUND.to_vec(),
                ..Default::default()
            };
            img = ops::flatten_with_opts(&img, &opts).map_err(engine_err("flatten"))?;
        }

        // 5) Codificación.
        let strip = strip_of(req.metadata);
        let q = req.quality.clamp(1, 100) as i32;
        match req.format {
            ImageFormat::Jpeg => {
                let o = ops::JpegsaveOptions {
                    q,
                    strip,
                    optimize_coding: true,
                    ..Default::default()
                };
                ops::jpegsave_with_opts(&img, output, &o)
            }
            ImageFormat::Png => {
                let bitdepth = if matches!(img.get_format(), Ok(BandFormat::Ushort)) {
                    16
                } else {
                    8
                };
                let o = ops::PngsaveOptions {
                    compression: 9,
                    bitdepth,
                    strip,
                    ..Default::default()
                };
                ops::pngsave_with_opts(&img, output, &o)
            }
            ImageFormat::Webp => {
                let o = ops::WebpsaveOptions {
                    q,
                    lossless: req.quality_mode == QualityMode::Lossless,
                    strip,
                    ..Default::default()
                };
                ops::webpsave_with_opts(&img, output, &o)
            }
        }
        .map_err(engine_err("encode"))?;

        Ok(EncodeOutput {
            width: img.get_width() as u32,
            height: img.get_height() as u32,
        })
    }
}
