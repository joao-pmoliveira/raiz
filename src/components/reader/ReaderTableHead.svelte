<script lang="ts">
    import type { TableHeadBlock } from "../../routes/library/library";
    import ReaderTableRow from "./ReaderTableRow.svelte";

    interface Props { tableHead: TableHeadBlock }

    let { tableHead }: Props = $props();

</script>

<thead>
    {#each tableHead.row as block, index (index)}
        <ReaderTableRow tableRow={block} isHeader={true}/>
    {/each}
</thead>

<style>

</style>
