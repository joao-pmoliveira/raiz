export interface Resource {
    document: Document;
    metadata: ResourceMetadata;
}

export interface ResourceMetadata {
    uuid: string;
    title: string;
    resource_type: ResourceType;
    path: string;
}

export enum ResourceType {
    Markdown = "Markdown",
}

export interface Document {
    blocks: DocumentBlock[];
}

export type DocumentBlock =
    | string
    | { Text: string }
    | { InlineCode: string }
    | { Heading: HeadingBlock }
    | { Paragraph: ParagraphBlock }
    | { BlockQuote: BlockQuoteBlock }
    | { CodeBlock: CodeBlockBlock }
    | { List: ListBlock }
    | { ListItem: ListItemBlock }
    | { Link: LinkBlock }
    | { Styled: StyledBlock }
    | { Table: TableBlock }
    | { TableHead: TableHeadBlock }
    | { TableRow: TableRowBlock }
    | { TableCell: TableCellBlock }
    | { ThematicBreak: { _: string } }
    | { SoftBreak: { _: string } }
    | { HardBreak: { _: string } };

export interface HeadingBlock {
    level: 1 | 2 | 3 | 4 | 5 | 6;
    children: DocumentBlock[];
}
export interface ParagraphBlock {
    children: DocumentBlock[];
}
export interface CodeBlockBlock {
    language: string;
    lines: string[];
}
export interface BlockQuoteBlock {
    kind: BlockQuoteBlockKind;
    children: DocumentBlock[];
}
export interface TableBlock {
    head: TableHeadBlock;
    rows: TableRowBlock[];
}
export interface TableHeadBlock {
    row: TableRowBlock[];
}
export interface TableRowBlock {
    cells: TableCellBlock[];
}
export interface TableCellBlock {
    children: DocumentBlock[];
}
export interface ListBlock {
    kind: ListItemBlockKind;
    items: ListItemBlock[];
}
export interface ListItemBlock {
    children: DocumentBlock[];
}
export interface LinkBlock {
    destination: string;
    children: DocumentBlock[];
}
export interface StyledBlock {
    style: StyledKind;
    children: DocumentBlock[];
}
export interface ListItemBlock {
    children: DocumentBlock[];
}

export enum StyledKind {
    Emphasis = "Emphasis",
    Strong = "Strong",
    Strikethrough = "Strikethrough",
    Superscript = "Superscript",
    Subscript = "Subscript",
}

export enum BlockQuoteBlockKind {
    Note = "Note",
    Tip = "Tip",
    Important = "Important",
    Warning = "Warning",
    Caution = "Caution",
    None = "None",
}

export type ListItemBlockKind = { Ordered: { start: number } } | { Unordered: { _: string } };
