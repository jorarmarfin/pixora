//! Cálculo puro de dimensiones de salida (sin dependencias del motor).

use crate::domain::{FitMode, JobError, JobErrorCode, ResizeOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub width: u32,
    pub height: u32,
    /// Escalar para cubrir y recortar al centro hasta width×height exactos.
    pub crop: bool,
}

const MAX_SIDE: f64 = 100_000.0;

fn scaled(v: u32, f: f64) -> u32 {
    ((v as f64 * f).round() as u32).max(1)
}

fn invalid(msg: &str) -> JobError {
    JobError::with_detail(JobErrorCode::InvalidOptions, msg)
}

/// Dimensiones finales a partir de las ya corregidas por orientación EXIF.
/// - `Contain`: cabe dentro de la caja manteniendo proporción (puede ampliar).
/// - `Fit`: como Contain pero nunca amplía.
/// - `Cover`: llena la caja y recorta al centro; tamaño exacto de la caja.
pub fn target_size(src_w: u32, src_h: u32, opts: &ResizeOptions) -> Result<Target, JobError> {
    if src_w == 0 || src_h == 0 {
        return Err(invalid("imagen sin dimensiones"));
    }
    let keep = |w: u32, h: u32| Target {
        width: w,
        height: h,
        crop: false,
    };
    let t = match *opts {
        ResizeOptions::None => keep(src_w, src_h),
        ResizeOptions::Width { px } => {
            if px == 0 {
                return Err(invalid("ancho 0"));
            }
            keep(px, scaled(src_h, px as f64 / src_w as f64))
        }
        ResizeOptions::Height { px } => {
            if px == 0 {
                return Err(invalid("alto 0"));
            }
            keep(scaled(src_w, px as f64 / src_h as f64), px)
        }
        ResizeOptions::Percent { value } => {
            if !(value.is_finite() && value > 0.0) {
                return Err(invalid("porcentaje inválido"));
            }
            let f = value as f64 / 100.0;
            keep(scaled(src_w, f), scaled(src_h, f))
        }
        ResizeOptions::Box { width, height, fit } => {
            if width == 0 || height == 0 {
                return Err(invalid("caja con lado 0"));
            }
            let fx = width as f64 / src_w as f64;
            let fy = height as f64 / src_h as f64;
            match fit {
                FitMode::Contain => {
                    let f = fx.min(fy);
                    keep(scaled(src_w, f), scaled(src_h, f))
                }
                FitMode::Fit => {
                    let f = fx.min(fy).min(1.0);
                    keep(scaled(src_w, f), scaled(src_h, f))
                }
                FitMode::Cover => Target {
                    width,
                    height,
                    crop: true,
                },
            }
        }
    };
    if t.width as f64 > MAX_SIDE || t.height as f64 > MAX_SIDE {
        return Err(JobError::new(JobErrorCode::TooLarge));
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bx(w: u32, h: u32, fit: FitMode) -> ResizeOptions {
        ResizeOptions::Box {
            width: w,
            height: h,
            fit,
        }
    }

    #[test]
    fn sin_cambios() {
        let t = target_size(800, 600, &ResizeOptions::None).unwrap();
        assert_eq!((t.width, t.height), (800, 600));
    }

    #[test]
    fn por_ancho_y_alto_mantienen_proporcion() {
        let t = target_size(800, 600, &ResizeOptions::Width { px: 400 }).unwrap();
        assert_eq!((t.width, t.height), (400, 300));
        let t = target_size(800, 600, &ResizeOptions::Height { px: 150 }).unwrap();
        assert_eq!((t.width, t.height), (200, 150));
    }

    #[test]
    fn porcentaje() {
        let t = target_size(801, 601, &ResizeOptions::Percent { value: 50.0 }).unwrap();
        assert_eq!((t.width, t.height), (401, 301));
        assert!(target_size(10, 10, &ResizeOptions::Percent { value: 0.0 }).is_err());
        assert!(target_size(10, 10, &ResizeOptions::Percent { value: f32::NAN }).is_err());
    }

    #[test]
    fn nunca_da_cero() {
        let t = target_size(1000, 1, &ResizeOptions::Width { px: 10 }).unwrap();
        assert_eq!((t.width, t.height), (10, 1));
    }

    #[test]
    fn contain_fit_y_cover() {
        let t = target_size(800, 600, &bx(400, 400, FitMode::Contain)).unwrap();
        assert_eq!((t.width, t.height, t.crop), (400, 300, false));
        // Contain amplía; Fit no.
        let t = target_size(100, 50, &bx(400, 400, FitMode::Contain)).unwrap();
        assert_eq!((t.width, t.height), (400, 200));
        let t = target_size(100, 50, &bx(400, 400, FitMode::Fit)).unwrap();
        assert_eq!((t.width, t.height), (100, 50));
        let t = target_size(800, 600, &bx(300, 300, FitMode::Cover)).unwrap();
        assert_eq!((t.width, t.height, t.crop), (300, 300, true));
    }

    #[test]
    fn rechaza_opciones_invalidas() {
        assert!(target_size(10, 10, &ResizeOptions::Width { px: 0 }).is_err());
        assert!(target_size(10, 10, &bx(0, 5, FitMode::Fit)).is_err());
        assert!(target_size(0, 10, &ResizeOptions::None).is_err());
        let e = target_size(10, 10, &ResizeOptions::Width { px: 2_000_000 }).unwrap_err();
        assert_eq!(e.code, JobErrorCode::TooLarge);
    }
}
