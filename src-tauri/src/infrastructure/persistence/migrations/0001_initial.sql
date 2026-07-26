CREATE TABLE resources (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    relative_path TEXT NOT NULL,
    file_extension TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
