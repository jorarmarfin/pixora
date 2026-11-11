use std::path::Path;

use crate::domain::{ImageFormat, JobError, MetadataPolicy, QualityMode, ResizeOptions};

/// Fondo con el que se aplana la transparencia al guardar en JPEG (blanco).
pub const ALPHA_BACKGROUND: [f64; 3] = [255.0, 255.0, 255.0];
/// Límite de píxeles de entrada para evitar agotar memoria (bombas de descompresión).
pub const MAX_INPUT_PIXELS: u64 = 250_000_000;

pub struct EncodeRequest<'a> {
    pub input: &'a Path,
    /// Archivo temporal de destino (lo gestiona el pipeline).
    pub output: &'a Path,
    pub format: ImageFormat,
    pub quality_mode: QualityMode,
    /// 1..=100, solo aplica a compresión con pérdida.
    pub quality: u8,
    pub resize: ResizeOptions,
    pub metadata: MetadataPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodeOutput {
    pub width: u32,
    pub height: u32,
}

/// Contrato del motor: decodifica, corrige orientación, redimensiona y codifica.
/// No toca nombres, colisiones ni limpieza de temporales (eso es del pipeline).
pub trait ImageEngine: Send + Sync {
    fn name(&self) -> &'static str;
    fn encode(&self, req: &EncodeRequest) -> Result<EncodeOutput, JobError>;
}
