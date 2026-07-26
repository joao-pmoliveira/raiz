use std::{
    fs::{copy, create_dir_all},
    path::Path,
};

use tauri::{AppHandle, Manager};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::{resource::ResourceMetadata, resource_type::ResourceType},
    library::RESOURCE_DIR,
};

pub fn copy_resource(app: &AppHandle, file_path: &Path) -> Result<ResourceMetadata, StorageError> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|_| StorageError::AppDataDirectoryUnavailable)?;

    let uuid = Uuid::new_v4();
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or(StorageError::MissingExtension)?;

    let resources_dir = dir.join(RESOURCE_DIR);
    create_dir_all(&resources_dir)?;

    let new_file_path = resources_dir.join(format!("{}.{}", uuid, extension));
    let relative_path = new_file_path
        .strip_prefix(&resources_dir)
        .expect("Copied file should be inside resources directory");

    copy(file_path, &new_file_path)?;

    let title = file_path
        .file_stem()
        .and_then(|fs| Some(fs.to_string_lossy()))
        .ok_or(StorageError::MissingFileName)?;

    Ok(ResourceMetadata {
        uuid,
        title: title.to_string(),
        resource_type: ResourceType::Markdown,
        path: relative_path.to_path_buf(),
    })
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Unable to determina the application data directory")]
    AppDataDirectoryUnavailable,

    #[error("Resource has no valid file extension")]
    MissingExtension,

    #[error("Resource has no valid filename")]
    MissingFileName,

    #[error("Filesystem erro")]
    Io(#[from] std::io::Error),
}
