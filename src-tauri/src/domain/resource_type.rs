use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ResourceType {
    Markdown,
}

impl ResourceType {
    pub fn id(&self) -> &'static str {
        match self {
            ResourceType::Markdown => "markdown",
        }
    }

    pub fn get_extension(&self) -> &'static str {
        match self {
            ResourceType::Markdown => "md",
        }
    }

    pub fn get_display(&self) -> &'static str {
        match self {
            ResourceType::Markdown => "Markdown",
        }
    }
}

impl ToSql for ResourceType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.id().into())
    }
}

impl FromSql for ResourceType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "markdown" => Ok(ResourceType::Markdown),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
