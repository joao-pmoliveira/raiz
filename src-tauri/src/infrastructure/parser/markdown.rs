use std::{fs::read_to_string, path::Path};

use pulldown_cmark::{
    BlockQuoteKind, CodeBlockKind,
    Event::{Code, End, HardBreak, Rule, SoftBreak, Start, Text},
    Options, Parser, Tag, TagEnd,
};
use tauri_plugin_log::log::info;
use thiserror::Error;

use crate::domain::document::{
    BlockQuoteKind as DocBlockQuoteKind, Document, DocumentBlock, InlineStyle, Item, ListKind,
    TCell, THead, TRow,
};
use DocumentBlock as Block;

pub fn import_md(path: &Path) -> Result<Document, ParserError> {
    let markdown = read_to_string(path)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown, options);

    let mut document = Document { blocks: Vec::new() };
    let mut stack: Vec<DocumentBlock> = Vec::new();

    for event in parser {
        match event {
            // __ START TAGS __
            Start(Tag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            }) => stack.push(Block::Heading {
                level: level as u8,
                children: Vec::new(),
            }),
            Start(Tag::Paragraph) => stack.push(Block::Paragraph {
                children: Vec::new(),
            }),
            Start(Tag::List(tag)) => {
                stack.push(Block::List {
                    kind: match tag {
                        Some(order) => ListKind::Ordered { start: order },
                        None => ListKind::Unordered {},
                    },
                    items: Vec::new(),
                });
            }
            Start(Tag::Item) => {
                stack.push(Block::ListItem {
                    children: Vec::new(),
                });
            }
            Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                stack.push(Block::CodeBlock {
                    language: None,
                    lines: Vec::new(),
                });
            }
            Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) => {
                stack.push(Block::CodeBlock {
                    language: Some(tag.into_string()),
                    lines: Vec::new(),
                });
            }
            Start(Tag::BlockQuote(kind)) => {
                stack.push(Block::BlockQuote {
                    kind: match kind {
                        Some(BlockQuoteKind::Note) => DocBlockQuoteKind::Note,
                        Some(BlockQuoteKind::Caution) => DocBlockQuoteKind::Caution,
                        Some(BlockQuoteKind::Important) => DocBlockQuoteKind::Important,
                        Some(BlockQuoteKind::Tip) => DocBlockQuoteKind::Tip,
                        Some(BlockQuoteKind::Warning) => DocBlockQuoteKind::Warning,
                        None => DocBlockQuoteKind::None,
                    },
                    children: Vec::new(),
                });
            }
            Start(Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            }) => {
                stack.push(Block::Link {
                    destination: dest_url.into_string(),
                    children: Vec::new(),
                });
            }
            Start(
                tag @ (Tag::Strong
                | Tag::Emphasis
                | Tag::Strikethrough
                | Tag::Superscript
                | Tag::Subscript),
            ) => {
                stack.push(Block::Styled {
                    style: match tag {
                        Tag::Strong => InlineStyle::Strong,
                        Tag::Emphasis => InlineStyle::Emphasis,
                        Tag::Strikethrough => InlineStyle::Strikethrough,
                        Tag::Superscript => InlineStyle::Superscript,
                        Tag::Subscript => InlineStyle::Subscript,
                        _ => unreachable!(),
                    },
                    children: Vec::new(),
                });
            }
            Start(Tag::Table(_)) => {
                stack.push(Block::Table {
                    head: None,
                    rows: Vec::new(),
                });
            }
            Start(Tag::TableHead) => {
                stack.push(Block::TableHead { rows: Vec::new() });
            }
            Start(Tag::TableRow) => {
                stack.push(Block::TableRow { cells: Vec::new() });
            }
            Start(Tag::TableCell) => {
                stack.push(Block::TableCell {
                    children: Vec::new(),
                });
            }
            Start(_) => (),

            // __ TEXT TAGS __
            Text(text) => match stack.last_mut() {
                Some(last) => {
                    last.append_text(text);
                }
                None => (),
            },
            Code(code) => match stack.last_mut() {
                Some(last) => {
                    last.append_inline_code(code);
                }
                None => (),
            },

            // __ SELF CONTAINED TAGS __
            Rule => {
                let n = Block::ThematicBreak {};
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.appendable_children() {
                            children.push(n);
                        } else {
                            unreachable!(
                                "Attempted to append child to leaf DocumentBlock: {:?}",
                                last
                            );
                        }
                    }
                    None => document.blocks.push(n),
                }
            }
            SoftBreak => {
                let n = Block::SoftBreak {};
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.appendable_children() {
                            children.push(n);
                        } else {
                            unreachable!(
                                "Attempted to append child to leaf DocumentBlock: {:?}",
                                last
                            );
                        }
                    }
                    None => document.blocks.push(n),
                }
            }
            HardBreak => {
                let n = Block::HardBreak {};
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.appendable_children() {
                            children.push(n);
                        } else {
                            unreachable!(
                                "Attempted to append child to leaf DocumentBlock: {:?}",
                                last
                            );
                        }
                    }
                    None => document.blocks.push(n),
                }
            }

            // __ END TAGS __
            End(TagEnd::Heading(_))
            | End(TagEnd::Paragraph)
            | End(TagEnd::List(_))
            | End(TagEnd::CodeBlock)
            | End(TagEnd::BlockQuote(_))
            | End(TagEnd::Link)
            | End(TagEnd::Strong)
            | End(TagEnd::Emphasis)
            | End(TagEnd::Strikethrough)
            | End(TagEnd::Superscript)
            | End(TagEnd::Subscript)
            | End(TagEnd::Table) => match stack.pop() {
                Some(block) => match stack.last_mut() {
                    Some(leaf) => {
                        if let Some(children) = leaf.appendable_children() {
                            children.push(block);
                        }
                    }
                    None => document.blocks.push(block),
                },
                None => unreachable!("Reading end tag for non-existent block"),
            },

            End(TagEnd::Item) => match stack.pop() {
                Some(Block::ListItem { children }) => match stack.last_mut() {
                    Some(Block::List { kind: _, items }) => {
                        items.push(Item { children });
                    }
                    Some(other) => unreachable!("Expected List, got {:?}", other),
                    None => unreachable!("Reading end tag for non-existent block"),
                },
                Some(other) => unreachable!("Expected List Item, got {:?}", other),
                None => unreachable!("Reading end tag for non-existent block"),
            },

            End(TagEnd::TableHead) => match stack.pop() {
                Some(Block::TableHead { rows }) => match stack.last_mut() {
                    Some(Block::Table { head, .. }) => {
                        assert!(head.is_none());
                        *head = Some(THead { row: rows });
                    }
                    Some(other) => unreachable!("Expected Table at stack end, got {:?}", other),
                    None => unreachable!("TableHead without parent Table"),
                },
                Some(other) => unreachable!("Expected TableHead, got {:?}", other),
                None => unreachable!("Reading end tag for non-existent block"),
            },

            End(TagEnd::TableRow) => match stack.pop() {
                Some(Block::TableRow { cells }) => match stack.last_mut() {
                    Some(Block::Table { head: _, rows }) | Some(Block::TableHead { rows }) => {
                        rows.push(TRow { cells });
                    }
                    Some(other) => {
                        unreachable!("Expected Table or TableHead at stack end, got {:?}", other)
                    }
                    None => unreachable!("TableHead without parent Table"),
                },
                Some(other) => unreachable!("Expected TableHead, got {:?}", other),
                None => unreachable!("Reading end tag for non-existent block"),
            },

            End(TagEnd::TableCell) => match stack.pop() {
                Some(Block::TableCell { children }) => {
                    info!("{:#?}", event);
                    match stack.last_mut() {
                        Some(Block::TableRow { cells }) => {
                            cells.push(TCell { children });
                        }
                        Some(Block::TableHead { rows }) => match rows.last_mut() {
                            Some(last_row) => last_row.cells.push(TCell { children }),
                            None => {
                                let mut new_row = TRow { cells: Vec::new() };
                                new_row.cells.push(TCell { children });
                                rows.push(new_row);
                            }
                        },
                        Some(other) => unreachable!("Expected Table at stack end, got {:?}", other),
                        None => unreachable!("TableHead without parent Table"),
                    }
                }
                Some(other) => unreachable!("Expected TableHead, got {:?}", other),
                None => unreachable!("Reading end tag for non-existent block"),
            },

            _ => (),
        }
    }

    Ok(document)
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),
}
