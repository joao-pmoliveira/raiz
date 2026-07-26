use std::path::PathBuf;

use serde::Serialize;
use uuid::Uuid;

use crate::domain::{document::Document, resource_type::ResourceType};

#[derive(Debug, Serialize)]
pub struct Resource {
    pub metadata: ResourceMetadata,
    pub document: Document,
}

#[derive(Debug, Serialize)]
pub struct ResourceMetadata {
    pub uuid: Uuid,
    pub title: String,
    pub resource_type: ResourceType,
    pub path: PathBuf,
}
