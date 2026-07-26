use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Document {
    pub blocks: Vec<DocumentBlock>,
}

#[derive(Debug, Serialize)]
pub enum DocumentBlock {
    Heading {
        level: u8,
        children: Vec<DocumentBlock>,
    },
    Paragraph {
        children: Vec<DocumentBlock>,
    },
    List {
        kind: ListKind,
        items: Vec<Item>,
    },
    ListItem {
        children: Vec<DocumentBlock>,
    },
    BlockQuote {
        kind: BlockQuoteKind,
        children: Vec<DocumentBlock>,
    },
    CodeBlock {
        language: Option<String>,
        lines: Vec<String>,
    },
    Link {
        destination: String,
        children: Vec<DocumentBlock>,
    },
    Styled {
        style: InlineStyle,
        children: Vec<DocumentBlock>,
    },
    Table {
        head: Option<THead>,
        rows: Vec<TRow>,
    },

    TableHead {
        rows: Vec<TRow>,
    },

    TableRow {
        cells: Vec<TCell>,
    },

    TableCell {
        children: Vec<DocumentBlock>,
    },
    Text(String),
    InlineCode(String),
    ThematicBreak {},
    SoftBreak {},
    HardBreak {},
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub children: Vec<DocumentBlock>,
}
#[derive(Debug, Serialize)]
pub enum ListKind {
    Unordered {},
    Ordered { start: u64 },
}
#[derive(Debug, Serialize)]
pub enum BlockQuoteKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
    None,
}
#[derive(Debug, Serialize)]
pub enum InlineStyle {
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
}
#[derive(Debug, Serialize)]
pub struct THead {
    pub row: Vec<TRow>,
}
#[derive(Debug, Serialize)]
pub struct TRow {
    pub cells: Vec<TCell>,
}
#[derive(Debug, Serialize)]
pub struct TCell {
    pub children: Vec<DocumentBlock>,
}

impl DocumentBlock {
    pub fn append_text(&mut self, text: impl Into<String>) {
        match self {
            DocumentBlock::Heading { level: _, children }
            | DocumentBlock::Paragraph { children }
            | DocumentBlock::BlockQuote { kind: _, children }
            | DocumentBlock::Link {
                destination: _,
                children,
            }
            | DocumentBlock::Styled { style: _, children }
            | DocumentBlock::TableCell { children }
            | DocumentBlock::ListItem { children } => {
                children.push(DocumentBlock::Text(text.into()));
            }
            DocumentBlock::CodeBlock { language: _, lines } => {
                lines.push(text.into());
            }
            // UNREACHABLES (keep for now for debugging purposes)
            DocumentBlock::Table { head: _, rows: __ } => {
                unreachable!("Cannot append text to <TABLE>")
            }
            DocumentBlock::List { kind: _, items: _ } => {
                unreachable!("Cannot append text to <LIST>")
            }
            DocumentBlock::TableHead { rows: _ } => {
                unreachable!("Cannot append text to <TABLE HEAD>")
            }
            DocumentBlock::TableRow { cells: _ } => {
                unreachable!("Cannot append text to <TABLE ROW>")
            }
            DocumentBlock::Text(_) => unreachable!("Cannot append text to <TEXT>"),
            DocumentBlock::InlineCode(_) => unreachable!("Cannot append text to <INLINE CODE>"),
            DocumentBlock::ThematicBreak {} => {
                unreachable!("Cannot append text to <THEMATIC BREAK>")
            }
            DocumentBlock::SoftBreak {} => unreachable!("Cannot append text to <SOFT BREAK>"),
            DocumentBlock::HardBreak {} => unreachable!("Cannot append text to <HARD BREAK>"),
        }
    }

    pub fn append_inline_code(&mut self, code: impl Into<String>) {
        match self {
            DocumentBlock::Heading { level: _, children }
            | DocumentBlock::Paragraph { children }
            | DocumentBlock::BlockQuote { kind: _, children }
            | DocumentBlock::Link {
                destination: _,
                children,
            }
            | DocumentBlock::Styled { style: _, children }
            | DocumentBlock::TableCell { children }
            | DocumentBlock::ListItem { children } => {
                children.push(DocumentBlock::InlineCode(code.into()));
            }
            DocumentBlock::CodeBlock { language: _, lines } => {
                lines.push(code.into());
            }
            // UNREACHABLES (keep for now for debugging purposes)
            DocumentBlock::Table { head: _, rows: __ } => {
                unreachable!("Cannot append inline code to <TABLE>")
            }
            DocumentBlock::List { kind: _, items: _ } => {
                unreachable!("Cannot append inline code to <LIST>")
            }
            DocumentBlock::TableHead { rows: _ } => {
                unreachable!("Cannot append inline code to <TABLE HEAD>")
            }
            DocumentBlock::TableRow { cells: _ } => {
                unreachable!("Cannot append inline code to <TABLE ROW>")
            }
            DocumentBlock::Text(_) => unreachable!("Cannot append inline code to <TEXT>"),
            DocumentBlock::InlineCode(_) => {
                unreachable!("Cannot append inline code to <INLINE CODE>")
            }
            DocumentBlock::ThematicBreak {} => {
                unreachable!("Cannot append inline code to <THEMATIC BREAK>")
            }
            DocumentBlock::SoftBreak {} => {
                unreachable!("Cannot append inline code to <SOFT BREAK>")
            }
            DocumentBlock::HardBreak {} => {
                unreachable!("Cannot append inline code to <HARD BREAK>")
            }
        }
    }

    pub fn appendable_children(&mut self) -> Option<&mut Vec<DocumentBlock>> {
        match self {
            DocumentBlock::Heading { level: _, children } => Some(children),
            DocumentBlock::Paragraph { children } => Some(children),
            DocumentBlock::BlockQuote { kind: _, children } => Some(children),
            DocumentBlock::Link {
                destination: _,
                children,
            } => Some(children),
            DocumentBlock::TableCell { children } => Some(children),
            DocumentBlock::Styled { style: _, children } => Some(children),

            _ => None,
        }
    }
}
