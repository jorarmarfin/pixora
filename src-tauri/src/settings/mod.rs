//! Preferencias y presets locales (JSON en el directorio de config del SO).
//!
//! Lectura tolerante: archivo ausente, corrupto o de versión desconocida → valores por defecto
//! (el archivo dañado se conserva como `.bak`). Escritura atómica (temporal + rename).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::{FitMode, ImageFormat, MetadataPolicy, QualityMode};
use crate::jobs;

pub const CONFIG_VERSION: u32 = 1;
const FILE: &str = "config.json";
const MAX_PRESETS: usize = 100;
const MAX_NAME: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResizeKind {
    None,
    Width,
    Height,
    Percent,
    Box,
}

/// Ajustes de conversión que forman un preset (espejo de los ajustes de la UI).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresetSettings {
    pub format: ImageFormat,
    pub quality_mode: QualityMode,
    pub quality: u8,
    pub resize_kind: ResizeKind,
    pub width: u32,
    pub height: u32,
    pub percent: f32,
    pub fit: FitMode,
    pub metadata: MetadataPolicy,
}

impl PresetSettings {
    /// Normaliza valores fuera de rango (config editada a mano o corrupta).
    fn sanitized(mut self) -> Self {
        self.quality = self.quality.clamp(1, 100);
        self.width = self.width.clamp(1, 100_000);
        self.height = self.height.clamp(1, 100_000);
        self.percent = if self.percent.is_finite() {
            self.percent.clamp(1.0, 1000.0)
        } else {
            100.0
        };
        if self.format == ImageFormat::Png {
            self.quality_mode = QualityMode::Lossless;
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub builtin: bool,
    pub settings: PresetSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub version: u32,
    pub theme: Theme,
    pub concurrency: usize,
    pub default_output_dir: String,
    pub last_preset_id: Option<String>,
    /// Solo presets del usuario; los integrados se añaden al cargar.
    pub presets: Vec<Preset>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            theme: Theme::Dark,
            concurrency: jobs::DEFAULT_CONCURRENCY,
            default_output_dir: String::new(),
            last_preset_id: None,
            presets: Vec::new(),
        }
    }
}

pub fn builtin_presets() -> Vec<Preset> {
    let base = PresetSettings {
        format: ImageFormat::Webp,
        quality_mode: QualityMode::Lossy,
        quality: 80,
        resize_kind: ResizeKind::None,
        width: 1920,
        height: 1080,
        percent: 50.0,
        fit: FitMode::Contain,
        metadata: MetadataPolicy::Strip,
    };
    let p = |id: &str, name: &str, settings| Preset {
        id: id.into(),
        name: name.into(),
        builtin: true,
        settings,
    };
    vec![
        p(
            "web",
            "Web optimizada",
            PresetSettings {
                resize_kind: ResizeKind::Width,
                ..base.clone()
            },
        ),
        p(
            "thumbnails",
            "Miniaturas",
            PresetSettings {
                quality: 75,
                resize_kind: ResizeKind::Box,
                width: 320,
                height: 320,
                fit: FitMode::Cover,
                ..base.clone()
            },
        ),
        p(
            "max-quality",
            "Máxima calidad",
            PresetSettings {
                quality_mode: QualityMode::Lossless,
                quality: 100,
                metadata: MetadataPolicy::KeepAll,
                ..base
            },
        ),
    ]
}

impl AppConfig {
    fn sanitized(mut self) -> Self {
        self.concurrency = jobs::clamp_concurrency(Some(self.concurrency));
        self.presets.retain(|p| !p.id.is_empty() && !p.builtin);
        self.presets.truncate(MAX_PRESETS);
        for p in &mut self.presets {
            p.name = p.name.trim().chars().take(MAX_NAME).collect();
            p.settings = p.settings.clone().sanitized();
        }
        self.version = CONFIG_VERSION;
        self
    }
}

/// Config + presets integrados, tal como la ve el frontend.
#[derive(Debug, Clone, Serialize)]
pub struct LoadedConfig {
    #[serde(flatten)]
    pub config: AppConfig,
    pub builtin_presets: Vec<Preset>,
    /// `true` si el archivo existente estaba dañado y se restauraron los valores por defecto.
    pub recovered: bool,
}

pub fn config_path(dir: &Path) -> PathBuf {
    dir.join(FILE)
}

pub fn load(dir: &Path) -> LoadedConfig {
    let path = config_path(dir);
    let (config, recovered) = match std::fs::read(&path) {
        Err(_) => (AppConfig::default(), false),
        Ok(bytes) => match serde_json::from_slice::<AppConfig>(&bytes) {
            Ok(c) if c.version <= CONFIG_VERSION => (c.sanitized(), false),
            _ => {
                let _ = std::fs::rename(&path, dir.join("config.json.bak"));
                (AppConfig::default(), true)
            }
        },
    };
    LoadedConfig {
        config,
        builtin_presets: builtin_presets(),
        recovered,
    }
}

pub fn save(dir: &Path, config: AppConfig) -> Result<AppConfig, String> {
    let config = config.sanitized();
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let tmp = dir.join(".config.json.tmp");
    let json = serde_json::to_vec_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&tmp, json).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, config_path(dir)).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        e.to_string()
    })?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_archivo_devuelve_defaults_y_presets_integrados() {
        let d = tempfile::tempdir().unwrap();
        let l = load(d.path());
        assert_eq!(l.config, AppConfig::default());
        assert!(!l.recovered);
        assert_eq!(l.builtin_presets.len(), 3);
        assert!(l.builtin_presets.iter().all(|p| p.builtin));
    }

    #[test]
    fn persiste_y_sobrevive_al_reinicio() {
        let d = tempfile::tempdir().unwrap();
        let mut c = AppConfig {
            theme: Theme::Light,
            concurrency: 4,
            default_output_dir: "/salida ñ".into(),
            last_preset_id: Some("u1".into()),
            ..Default::default()
        };
        c.presets.push(Preset {
            id: "u1".into(),
            name: "Mío".into(),
            builtin: false,
            settings: builtin_presets()[0].settings.clone(),
        });
        save(d.path(), c.clone()).unwrap();
        let l = load(d.path());
        assert_eq!(l.config, c);
        assert!(!d.path().join(".config.json.tmp").exists());
    }

    #[test]
    fn archivo_corrupto_se_respalda_y_se_restaura() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(config_path(d.path()), b"{ esto no es json").unwrap();
        let l = load(d.path());
        assert!(l.recovered);
        assert_eq!(l.config, AppConfig::default());
        assert!(d.path().join("config.json.bak").exists());
        assert!(!config_path(d.path()).exists());
    }

    #[test]
    fn version_futura_se_trata_como_ilegible() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(config_path(d.path()), br#"{"version": 99}"#).unwrap();
        assert!(load(d.path()).recovered);
    }

    #[test]
    fn campos_ausentes_toman_defaults_y_valores_invalidos_se_corrigen() {
        let d = tempfile::tempdir().unwrap();
        std::fs::write(
            config_path(d.path()),
            br#"{"theme":"light","concurrency":999,"presets":[
              {"id":"a","name":"  x  ","settings":{"format":"png","quality_mode":"lossy","quality":0,
               "resize_kind":"none","width":0,"height":5,"percent":-3,"fit":"fit","metadata":"strip"}},
              {"id":"","name":"sin id","settings":{"format":"png","quality_mode":"lossy","quality":1,
               "resize_kind":"none","width":1,"height":1,"percent":1,"fit":"fit","metadata":"strip"}}]}"#,
        )
        .unwrap();
        let l = load(d.path());
        assert!(!l.recovered);
        assert_eq!(l.config.theme, Theme::Light);
        assert_eq!(l.config.concurrency, jobs::MAX_CONCURRENCY);
        assert_eq!(l.config.presets.len(), 1, "preset sin id descartado");
        let p = &l.config.presets[0];
        assert_eq!(p.name, "x");
        assert_eq!((p.settings.quality, p.settings.width), (1, 1));
        assert_eq!(p.settings.percent, 1.0);
        assert_eq!(p.settings.quality_mode, QualityMode::Lossless, "PNG");
    }

    #[test]
    fn no_se_guardan_presets_integrados() {
        let d = tempfile::tempdir().unwrap();
        let c = AppConfig {
            presets: builtin_presets(),
            ..Default::default()
        };
        assert!(save(d.path(), c).unwrap().presets.is_empty());
    }
}
