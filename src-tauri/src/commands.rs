use tauri::State;
use tauri_plugin_dialog::DialogExt;
use uuid::Uuid;

use crate::{
    domain::{AppState, Library, Resource, ResourceMetadata, ResourceType},
    import::import_markdown,
};

#[tauri::command]
pub async fn select_resource(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Resource, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .blocking_pick_file()
        .ok_or("User cancelled file selection")?;

    let path = file
        .into_path()
        .map_err(|_| "Unable to load selected resource".to_string())?;

    let document = import_markdown(&path);

    let resource = Resource {
        metadata: ResourceMetadata {
            id: Uuid::new_v4(),
            title: String::from(path.file_stem().unwrap().to_string_lossy()),
            resource_type: ResourceType::Markdown,
            file_name: String::from(path.file_name().unwrap().to_string_lossy()),
        },
        content: document,
    };

    {
        let mut library = state.library.lock().await;
        library.resources.push(resource.clone());
    }

    Ok(resource)
}

#[tauri::command]
pub async fn get_library(state: State<'_, AppState>) -> Result<Library, String> {
    let library = state.library.lock().await;

    Ok(library.clone())
}

#[tauri::command]
pub async fn get_resource(
    resource_id: Uuid,
    state: State<'_, AppState>,
) -> Result<Resource, String> {
    let library = state.library.lock().await;

    let resource = library
        .resources
        .iter()
        .find(|r| r.metadata.id == resource_id)
        .cloned()
        .ok_or_else(|| "Resource not found".to_string())?;

    Ok(resource)
}
