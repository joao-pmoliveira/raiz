<script lang="ts">
    import type { DocumentBlock } from "../../routes/library/library";
    import ReaderText from "./ReaderText.svelte";
    import ReaderCode from "./ReaderCode.svelte";
    import ReaderHeading from "./ReaderHeading.svelte";
    import ReaderParagraph from "./ReaderParagraph.svelte";
    import ReaderCodeBlock from "./ReaderCodeBlock.svelte";
    import ReaderLink from "./ReaderLink.svelte";
    import ReaderStyled from "./ReaderStyled.svelte";
    import ReaderQuoteBlock from "./ReaderQuoteBlock.svelte";
    import ReaderTable from "./ReaderTable.svelte";
    import ReaderList from "./ReaderList.svelte";

    interface Props { block : DocumentBlock}

    let { block } : Props = $props();

</script>


{#if typeof block === "string"}
    {block}
{:else if "Text" in block}
    <ReaderText text={block.Text} />
{:else if "InlineCode" in block}
    <ReaderCode code={block.InlineCode} />
{:else if "Heading" in block}
    <ReaderHeading heading={block.Heading} />
{:else if "Paragraph" in block}
    <ReaderParagraph paragraph={block.Paragraph} />
{:else if "CodeBlock" in block}
    <ReaderCodeBlock codeblock={block.CodeBlock} />
{:else if "Link" in block}
    <ReaderLink link={block.Link} />
{:else if "Styled" in block}
    <ReaderStyled styled={block.Styled} />
{:else if "BlockQuote" in block}
    <ReaderQuoteBlock blockQuote={block.BlockQuote} />
{:else if "List" in block}
    <ReaderList list={block.List} />
{:else if "Table" in block}
    <ReaderTable table={block.Table} />
{:else if "SoftBreak" in block}
    <br>
{:else if "HardBreak" in block}
    <br>
{:else if "ThematicBreak" in block}
    <hr>
{/if}
