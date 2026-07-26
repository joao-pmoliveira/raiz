<script lang="ts">
    import type { TableRowBlock } from "../../routes/library/library";
    import ReaderTableCell from "./ReaderTableCell.svelte";

    interface Props { tableRow: TableRowBlock, isHeader?: boolean }

    let { tableRow, isHeader = false }: Props = $props();
</script>

<tr>
    {#each tableRow.cells as block, index (index)}
        <ReaderTableCell tableCell={block} {isHeader}/>
    {/each}
</tr>


<style>

</style>
