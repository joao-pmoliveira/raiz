<script lang="ts">
    import type { DocumentBlock } from "../../routes/library/library";
    import ReaderCode from "./ReaderCode.svelte";
    import ReaderCodeBlock from "./ReaderCodeBlock.svelte";
    import ReaderEmphasis from "./ReaderEmphasis.svelte";
    import ReaderHeading from "./ReaderHeading.svelte";
    import ReaderLink from "./ReaderLink.svelte";
    import ReaderListItem from "./ReaderListItem.svelte";
    import ReaderParagraph from "./ReaderParagraph.svelte";
    import ReaderQuoteBlock from "./ReaderQuoteBlock.svelte";
    import ReaderStrong from "./ReaderStrong.svelte";
    import ReaderTable from "./ReaderTable.svelte";
    import ReaderTableCell from "./ReaderTableCell.svelte";
    import ReaderTableHead from "./ReaderTableHead.svelte";
    import ReaderTableRow from "./ReaderTableRow.svelte";
    import ReaderText from "./ReaderText.svelte";


    interface Props { block : DocumentBlock}

    let { block } : Props = $props();
</script>


{#if "Text" in block}
<ReaderText text={block.Text}/>
{:else if "Code" in block}
<ReaderCode code={block.Code}/>
{:else if "Heading" in block}
<ReaderHeading heading={block.Heading} />
{:else if "Paragraph" in block}
<ReaderParagraph paragraph={block.Paragraph} />
{:else if "ListItem" in block}
<ReaderListItem listItem={block.ListItem} />
{:else if "CodeBlock" in block}
<ReaderCodeBlock codeblock={block.CodeBlock} />
{:else if "BlockQuote" in block}
<ReaderQuoteBlock blockQuote={block.BlockQuote} />
{:else if "Link" in block}
<ReaderLink link={block.Link}/>
{:else if "Strong" in block}
<ReaderStrong strong={block.Strong}/>
{:else if "Emphasis" in block}
<ReaderEmphasis emphasis={block.Emphasis}/>
{:else if "Table" in block}
<ReaderTable table={block.Table}/>
{:else if "TableHead" in block}
<ReaderTableHead tableHead={block.TableHead}/>
{:else if "TableRow" in block}
<ReaderTableRow tableRow={block.TableRow}/>
{:else if "TableCell" in block}
<ReaderTableCell tableCell={block.TableCell}/>
{:else if "Ruler" in block}
<hr>
{:else if "SoftBreak" in block}
<br>
{:else if "HardBreak" in block}
<br>
{/if}


<style>

</style>
