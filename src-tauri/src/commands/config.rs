//! Preferencias y presets persistentes.

use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::settings::{self, AppConfig, LoadedConfig};

fn dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path().app_config_dir().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_config(app: AppHandle) -> Result<LoadedConfig, String> {
    Ok(settings::load(&dir(&app)?))
}

/// Guarda la config completa (la UI es la única escritora). Devuelve la versión saneada.
#[tauri::command]
pub fn save_config(app: AppHandle, config: AppConfig) -> Result<AppConfig, String> {
    settings::save(&dir(&app)?, config)
}
