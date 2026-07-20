export interface Library {
    resources: Resource[];
}

export interface Resource {
    content: Document;
    metadata: ResourceMetadata;
}

export interface Document {
    blocks: DocumentBlock[];
}
export interface ResourceMetadata {
    id: string;
    title: string;
    file_name: string;
    resource_type: ResourceType;
}

enum ResourceType {
    Markdown = "Markdown",
}

export type DocumentBlock =
    | { Text: string }
    | { Code: string }
    | { Heading: Heading }
    | { Paragraph: Paragraph }
    | { ListItem: ListItem }
    | { CodeBlock: CodeBlock }
    | { BlockQuote: BlockQuote }
    | { Link: Link }
    | { Strong: Strong }
    | { Emphasis: Emphasis }
    | { Table: Table }
    | { TableHead: TableHead }
    | { TableRow: TableRow }
    | { TableCell: TableCell }
    | { Ruler: Ruler };

export interface Heading {
    type: "heading";
    level: 1 | 2 | 3 | 4 | 5 | 6;
    children: DocumentBlock[];
}

export interface Paragraph {
    type: "paragraph";
    children: DocumentBlock[];
}

export interface ListItem {
    type: "list-item";
    children: DocumentBlock[];
}

export interface CodeBlock {
    codeblock_type: null | { Fenced: string };
    children: DocumentBlock[];
}
export interface BlockQuote {
    blockquote_type: "None" | "Note" | "Tip" | "Important" | "Warning" | "Caution";
    children: DocumentBlock[];
}

export interface Link {
    type: "link";
    destination_url: string;
    children: DocumentBlock[];
    id: string;
}

export interface Strong {
    type: "strong";
    children: DocumentBlock[];
}
export interface Emphasis {
    type: "emphasis";
    children: DocumentBlock[];
}

export interface Table {
    type: "table";
    children: DocumentBlock[];
}
export interface TableHead {
    type: "table-head";
    children: DocumentBlock[];
}
export interface TableRow {
    type: "table-row";
    children: DocumentBlock[];
}
export interface TableCell {
    type: "table-cell";
    children: DocumentBlock[];
}
export interface Ruler {
    type: "ruler";
}
export interface SoftBreak {
    type: "soft-break";
}
export interface HardBreak {
    type: "hard-break";
}
