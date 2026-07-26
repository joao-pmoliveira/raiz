<script lang="ts">
    import type { TableCellBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { tableCell: TableCellBlock, isHeader?: boolean}

    let { tableCell, isHeader = false}: Props = $props();

</script>

{#if isHeader}
    <th>
        {#each tableCell.children as block, index (index)}
            <ReaderRendered {block} />
        {/each}
    </th>
{:else}
    <td>
        {#each tableCell.children as block, index (index)}
            <ReaderRendered {block} />
        {/each}
    </td>
{/if}

<style>
    td{
        border: 1px hsl(0, 0%, 15%) solid;
        padding: 0.3rem 1rem 0.4rem 1rem;
    }
    th {
        border: 1px hsl(0, 0%, 15%) solid;
        padding: 0.3rem 1rem 0.4rem 1rem;
    }
</style>
