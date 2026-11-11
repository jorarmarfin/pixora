//! Abrir una carpeta en el explorador del sistema.

use std::path::Path;
use std::process::Command;

/// Solo abre directorios existentes (nunca ejecuta archivos).
#[tauri::command]
pub fn reveal_folder(path: String) -> Result<(), String> {
    let p = std::fs::canonicalize(Path::new(&path)).map_err(|e| e.to_string())?;
    if !p.is_dir() {
        return Err("no es una carpeta".into());
    }
    #[cfg(target_os = "linux")]
    let mut cmd = Command::new("xdg-open");
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    cmd.arg(&p).spawn().map(|_| ()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rechaza_archivos_e_inexistentes() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("a.txt");
        std::fs::write(&f, "x").unwrap();
        assert!(reveal_folder(f.to_string_lossy().into_owned()).is_err());
        assert!(reveal_folder(d.path().join("no").to_string_lossy().into_owned()).is_err());
    }
}
