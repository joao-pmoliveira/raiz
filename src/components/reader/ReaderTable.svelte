<script lang="ts">
    import type { TableBlock } from "../../routes/library/library";
    import ReaderTableHead from "./ReaderTableHead.svelte";
    import ReaderTableRow from "./ReaderTableRow.svelte";

    interface Props { table: TableBlock }

    let { table }: Props = $props();
</script>

<table>
    <ReaderTableHead tableHead={table.head}/>

    <tbody>
    {#each table.rows as block, index (index)}
        <ReaderTableRow tableRow={block} />
    {/each}
    </tbody>
</table>


<style>
</style>
