<script lang="ts">
    import type { TableCell } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { tableCell: TableCell }

    let { tableCell }: Props = $props();

</script>

<td>
    {#each tableCell.children as block, index (index)}
        <ReaderRendered {block}/>
    {/each}
</td>


<style>

</style>
