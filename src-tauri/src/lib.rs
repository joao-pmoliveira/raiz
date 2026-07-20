use tauri::async_runtime::Mutex;

use crate::domain::AppState;
use crate::domain::Library;

pub mod commands;
pub mod domain;
pub mod import;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        library: Mutex::new(Library {
            resources: Vec::new(),
        }),
    };

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::select_resource,
            commands::get_library,
            commands::get_resource
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
