use std::path::PathBuf;

use rusqlite::params;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    domain::resource::ResourceMetadata,
    infrastructure::persistence::database::{open_database_connection, DatabaseError},
};

pub fn create_resource(app: &AppHandle, resource: ResourceMetadata) -> Result<(), DatabaseError> {
    let conn = open_database_connection(app)?;

    conn.execute(
        "INSERT INTO resources (id, title, resource_type, relative_path, file_extension) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            resource.uuid.to_string(),
            resource.title,
            resource.resource_type,
            resource.path.to_string_lossy(),
            resource.resource_type.get_extension()
        ]
    )?;

    Ok(())
}

pub fn find_all(app: &AppHandle) -> Result<Vec<ResourceMetadata>, DatabaseError> {
    let conn = open_database_connection(app)?;

    let mut stmt = conn
        .prepare("SELECT id, title, resource_type, relative_path, file_extension FROM resources")?;

    let mapped_rows = stmt.query_map([], |row| {
        Ok(ResourceMetadata {
            uuid: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
            title: row.get("title")?,
            resource_type: row.get("resource_type")?,
            path: PathBuf::from(row.get::<_, String>("relative_path")?),
        })
    })?;

    let resources = mapped_rows.collect::<Result<Vec<ResourceMetadata>, _>>()?;

    Ok(resources)
}

pub fn find(app: &AppHandle, uuid: Uuid) -> Result<ResourceMetadata, DatabaseError> {
    let conn = open_database_connection(app)?;

    let mut stmt = conn
        .prepare("SELECT id, title, resource_type, relative_path FROM resources WHERE id = ?1")?;

    let resource = stmt.query_row([uuid.to_string()], |row| {
        Ok(ResourceMetadata {
            uuid: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
            title: row.get("title")?,
            resource_type: row.get("resource_type")?,
            path: PathBuf::from(row.get::<_, String>("relative_path")?),
        })
    })?;

    Ok(resource)
}
