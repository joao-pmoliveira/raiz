use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{
    domain::{
        document::Document,
        resource::{Resource, ResourceMetadata},
    },
    infrastructure::{
        parser::markdown::import_md,
        persistence::library_repository::{create_resource, find, find_all},
        storage::storage::copy_resource,
    },
    library::RESOURCE_DIR,
};

#[tauri::command]
pub async fn select_resource(app: tauri::AppHandle) -> Result<(), String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .blocking_pick_file()
        .ok_or("User cancelled file selection")?;

    let path = file
        .into_path()
        .map_err(|_| "Unable to load selected resource".to_string())?;

    let _supported_extension = match path.extension().and_then(|ext| ext.to_str()) {
        Some("md") => Ok(true),
        Some(ext) => Err(format!("Unsupported extension: {}", ext)),
        None => Err(String::from("File has no extension")),
    }?;

    let resource = copy_resource(&app, path.as_path()).map_err(|er| er.to_string())?;

    create_resource(&app, resource).map_err(|er| er.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_library(app: tauri::AppHandle) -> Result<Vec<ResourceMetadata>, String> {
    find_all(&app).map_err(|er| er.to_string())
}

#[tauri::command]
pub async fn get_resource(app: tauri::AppHandle, uuid: Uuid) -> Result<Resource, String> {
    let metadata = find(&app, uuid).map_err(|er| er.to_string())?;

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "unable to get app dir")?;

    let path = data_dir.join(RESOURCE_DIR).join(&metadata.path);

    let document = import_md(&path).map_err(|er| er.to_string())?;

    let resource = Resource { metadata, document };
    Ok(resource)
}
