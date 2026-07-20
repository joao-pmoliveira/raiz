use crate::domain::{
    Document,
    DocumentBlock::{self},
    QuoteType,
};
use pulldown_cmark::{
    BlockQuoteKind, CodeBlockKind, Event, Options, Parser,
    Tag::{self},
    TagEnd,
};
use std::{fs::read_to_string, path::Path};

pub fn import_markdown(path: &Path) -> Document {
    let markdown = read_to_string(path).unwrap();
    parse_markdown(&markdown)
}

pub fn parse_markdown(markdown: &str) -> Document {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, options);

    let mut document = Document { blocks: Vec::new() };

    let mut stack: Vec<DocumentBlock> = Vec::new();

    for event in parser {
        match event {
            // START TAGS =========================================
            Event::Start(Tag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            }) => {
                stack.push(DocumentBlock::Heading {
                    level: level as u8,
                    children: Vec::new(),
                });
            }
            Event::Start(Tag::Paragraph) => stack.push(DocumentBlock::Paragraph {
                children: Vec::new(),
            }),
            Event::Start(Tag::Item) => stack.push(DocumentBlock::ListItem {
                children: Vec::new(),
            }),
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => {
                stack.push(DocumentBlock::CodeBlock {
                    codeblock_type: String::new(),
                    children: Vec::new(),
                })
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) => {
                stack.push(DocumentBlock::CodeBlock {
                    codeblock_type: tag.into_string(),
                    children: Vec::new(),
                })
            }
            Event::Start(Tag::BlockQuote(kind)) => stack.push(DocumentBlock::BlockQuote {
                quote_type: match kind {
                    Some(BlockQuoteKind::Note) => QuoteType::Note,
                    Some(BlockQuoteKind::Caution) => QuoteType::Caution,
                    Some(BlockQuoteKind::Important) => QuoteType::Important,
                    Some(BlockQuoteKind::Tip) => QuoteType::Tip,
                    Some(BlockQuoteKind::Warning) => QuoteType::Warning,
                    None => QuoteType::None,
                },
                children: Vec::new(),
            }),
            Event::Start(Tag::Link {
                link_type: _,
                dest_url,
                title: _,
                id: _,
            }) => stack.push(DocumentBlock::Link {
                destination_url: dest_url.into_string(),
                children: Vec::new(),
            }),
            Event::Start(Tag::Strong) => stack.push(DocumentBlock::Strong {
                children: Vec::new(),
            }),
            Event::Start(Tag::Emphasis) => stack.push(DocumentBlock::Emphasis {
                children: Vec::new(),
            }),
            Event::Start(Tag::Table(_alignment)) => stack.push(DocumentBlock::Table {
                children: Vec::new(),
            }),
            Event::Start(Tag::TableHead) => stack.push(DocumentBlock::TableHead {
                children: Vec::new(),
            }),
            Event::Start(Tag::TableRow) => stack.push(DocumentBlock::TableRow {
                children: Vec::new(),
            }),
            Event::Start(Tag::TableCell) => stack.push(DocumentBlock::TableCell {
                children: Vec::new(),
            }),
            // TEXT TAGS =========================================
            Event::Text(text) => {
                let n = DocumentBlock::Text(text.into_string());

                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.children_mut() {
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
            Event::Code(text) => {
                let n = DocumentBlock::Code(text.into_string());
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.children_mut() {
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

            // END TAGS =========================================
            Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::CodeBlock)
            | Event::End(TagEnd::Link)
            | Event::End(TagEnd::Strong)
            | Event::End(TagEnd::Emphasis)
            | Event::End(TagEnd::Table)
            | Event::End(TagEnd::TableHead)
            | Event::End(TagEnd::TableRow)
            | Event::End(TagEnd::TableCell)
            | Event::End(TagEnd::BlockQuote(_)) => {
                let block = stack.pop();
                match stack.last_mut() {
                    Some(leaf) => {
                        if let Some(children) = leaf.children_mut() {
                            children.push(block.unwrap());
                        }
                    }
                    None => document.blocks.push(block.unwrap()),
                }
            }

            // OTHER TAGS =========================================
            Event::Rule => {
                let n = DocumentBlock::Ruler(1);

                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.children_mut() {
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
            Event::SoftBreak => {
                let n = DocumentBlock::SoftBreak(1);
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.children_mut() {
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
            Event::HardBreak => {
                let n = DocumentBlock::HardBreak(1);
                match stack.last_mut() {
                    Some(last) => {
                        if let Some(children) = last.children_mut() {
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
            _ => {}
        }
    }

    document
}
