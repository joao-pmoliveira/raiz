<script lang="ts">
    import type { TableRow } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { tableRow: TableRow }

    let { tableRow }: Props = $props();

</script>

<tr>
    {#each tableRow.children as block, index (index)}
        <ReaderRendered {block}/>
    {/each}
</tr>


<style>

</style>
