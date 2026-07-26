use tauri::async_runtime::Mutex;

pub mod commands;
pub mod domain;
pub mod infrastructure;
pub mod library;

/*
    TODO 2: update front-end to use new resources correctly when opening reader
    TODO 3: tighten resouce type in both backend and frontend
    --- committ ---
    008 - Persist Reading Position
    TODO 4: check with chat with there's a good way / purpose to tackling this now, or if it should be left for after.
        - also talk about a potential good method for subdividing pages (blocks) into reading chunks. Will benefit both .md as well as epubs
    --- ? ---
    Decide if it's better to go into Epubs or better to finish the other main features such as:
        - hover effect on words, and search for them on database (lookup familiarity and such)
        - update familiarity on interaction

*/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
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
