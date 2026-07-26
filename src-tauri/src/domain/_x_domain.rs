use std::fmt::Display;

use serde::Serialize;
use tauri::async_runtime::Mutex;
use uuid::Uuid;

impl DocumentBlock {
    pub fn children_mut(&mut self) -> Option<&mut Vec<DocumentBlock>> {
        match self {
            DocumentBlock::Heading { level: _, children } => Some(children),
            DocumentBlock::Paragraph { children } => Some(children),
            DocumentBlock::List { order: _, children } => Some(children),
            DocumentBlock::ListItem { children } => Some(children),
            DocumentBlock::BlockQuote {
                quote_type: _,
                children,
            } => Some(children),
            DocumentBlock::CodeBlock {
                codeblock_type: _,
                children,
            } => Some(children),
            DocumentBlock::Emphasis { children } => Some(children),
            DocumentBlock::Strong { children } => Some(children),
            DocumentBlock::Strikethrough { children } => Some(children),
            DocumentBlock::Superscript { children } => Some(children),
            DocumentBlock::Subscript { children } => Some(children),
            DocumentBlock::Link {
                destination_url: _,
                children,
            } => Some(children),
            DocumentBlock::Table { children } => Some(children),
            DocumentBlock::TableHead { children } => Some(children),
            DocumentBlock::TableRow { children } => Some(children),
            DocumentBlock::TableCell { children } => Some(children),
            DocumentBlock::Text(_) => None,
            DocumentBlock::Code(_) => None,
            DocumentBlock::Ruler(_) => None,
            DocumentBlock::SoftBreak(_) => None,
            DocumentBlock::HardBreak(_) => None,
        }
    }
}
