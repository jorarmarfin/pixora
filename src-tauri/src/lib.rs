//! Pixora — núcleo Rust (comandos Tauri, motor de imagen, cola de trabajos).

pub mod commands;
pub mod domain;
pub mod filesystem;
pub mod imaging;
pub mod jobs;
pub mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(commands::batch::BatchState::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::app::app_info,
            commands::import::import_paths,
            commands::import::get_thumbnail,
            commands::process::process_single,
            commands::batch::start_batch,
            commands::batch::cancel_batch,
            commands::reveal::reveal_folder,
            commands::config::load_config,
            commands::config::save_config
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar Pixora");
}
