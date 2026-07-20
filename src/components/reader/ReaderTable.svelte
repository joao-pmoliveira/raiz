<script lang="ts">
    import type { Table } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { table: Table }

    let { table }: Props = $props();

</script>

<table>
    {#each table.children as block, index (index)}
        <ReaderRendered {block} />
    {/each}
</table>


<style>

</style>
