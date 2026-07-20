use serde::Serialize;
use tauri::async_runtime::Mutex;
use uuid::Uuid;

pub struct AppState {
    pub library: Mutex<Library>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Library {
    pub resources: Vec<Resource>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Resource {
    pub metadata: ResourceMetadata,
    pub content: Document,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResourceMetadata {
    pub id: Uuid,
    pub title: String,
    pub file_name: String,
    pub resource_type: ResourceType,
}

#[derive(Debug, Serialize, Clone)]
pub enum ResourceType {
    Markdown,
}

#[derive(Debug, Serialize, Clone)]
pub struct Document {
    pub blocks: Vec<DocumentBlock>,
}

#[derive(Debug, Serialize, Clone)]
pub enum DocumentBlock {
    Heading {
        level: u8,
        children: Vec<DocumentBlock>,
    },
    Paragraph {
        children: Vec<DocumentBlock>,
    },
    List {
        order: u64,
        children: Vec<DocumentBlock>,
    },
    ListItem {
        children: Vec<DocumentBlock>,
    },
    BlockQuote {
        quote_type: QuoteType,
        children: Vec<DocumentBlock>,
    },
    CodeBlock {
        codeblock_type: String,
        children: Vec<DocumentBlock>,
    },
    Link {
        destination_url: String,
        children: Vec<DocumentBlock>,
    },
    Emphasis {
        children: Vec<DocumentBlock>,
    },
    Strong {
        children: Vec<DocumentBlock>,
    },
    Strikethrough {
        children: Vec<DocumentBlock>,
    },
    Superscript {
        children: Vec<DocumentBlock>,
    },
    Subscript {
        children: Vec<DocumentBlock>,
    },
    Table {
        children: Vec<DocumentBlock>,
    },
    TableHead {
        children: Vec<DocumentBlock>,
    },
    TableRow {
        children: Vec<DocumentBlock>,
    },
    TableCell {
        children: Vec<DocumentBlock>,
    },
    Text(String),
    Code(String),
    Ruler(u8),
    SoftBreak(u8),
    HardBreak(u8),
}

#[derive(Debug, Serialize, Clone)]
pub enum QuoteType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
    None,
}

#[derive(Debug, Serialize, Clone)]
pub enum CodeBlockType {
    Indented,
    Fenced(String),
}

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
