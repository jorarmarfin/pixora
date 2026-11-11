//! Modelos y reglas de dominio, independientes de la UI y del motor.

use serde::{Deserialize, Serialize};

/// Estados de una tarea: queued -> validating -> processing -> terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Validating,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Skipped,
}

impl JobStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::Skipped
        )
    }
}

/// Formatos de entrada soportados en el MVP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
}

impl ImageFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }
}

/// Imagen importada con sus metadatos (lectura de cabecera, sin decodificar).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ImageItem {
    /// Ruta canónica; sirve también de identificador único.
    pub id: String,
    pub path: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub size_bytes: u64,
}

/// Archivo que no se pudo importar, con motivo estructurado.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SkippedFile {
    pub name: String,
    pub reason: SkipReason,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkipReason {
    Unsupported,
    Unreadable,
    Corrupt,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct ImportResult {
    pub items: Vec<ImageItem>,
    pub skipped: Vec<SkippedFile>,
}

// ---- Contrato de motor (se implementa en la Fase 3) ----

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityMode {
    Lossy,
    Lossless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FitMode {
    Contain,
    Cover,
    Fit,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResizeOptions {
    None,
    Width {
        px: u32,
    },
    Height {
        px: u32,
    },
    Percent {
        value: f32,
    },
    Box {
        width: u32,
        height: u32,
        fit: FitMode,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictPolicy {
    Rename,
    Skip,
    Overwrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataPolicy {
    Strip,
    KeepIcc,
    KeepAll,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageJob {
    pub id: String,
    pub input_path: String,
    pub output_directory: String,
    pub output_format: ImageFormat,
    pub quality_mode: QualityMode,
    pub quality_value: u8,
    pub resize: ResizeOptions,
    pub metadata_policy: MetadataPolicy,
    pub name_suffix: String,
    pub conflict_policy: ConflictPolicy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_es_insensible_a_mayusculas() {
        assert_eq!(ImageFormat::from_extension("JPG"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("WebP"), Some(ImageFormat::Webp));
        assert_eq!(ImageFormat::from_extension("gif"), None);
    }

    #[test]
    fn estados_terminales() {
        assert!(JobStatus::Failed.is_terminal());
        assert!(!JobStatus::Processing.is_terminal());
        assert_eq!(
            serde_json::to_string(&JobStatus::Queued).unwrap(),
            "\"queued\""
        );
    }
}
