use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub engine: &'static str,
}

pub fn build_app_info() -> AppInfo {
    AppInfo {
        name: "Pixora",
        version: env!("CARGO_PKG_VERSION"),
        // Motor activo desde la Fase 3.
        engine: "libvips",
    }
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    build_app_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_reporta_nombre_y_version() {
        let info = build_app_info();
        assert_eq!(info.name, "Pixora");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
    }
}
