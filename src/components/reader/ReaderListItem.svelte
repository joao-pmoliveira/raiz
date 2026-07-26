<script lang="ts">
    import type { ListItemBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { listItem: ListItemBlock };

    let { listItem }: Props = $props();
</script>

<li>
    {#each listItem.children as block, index (index)}
        <ReaderRendered {block} />
    {/each}
</li>


<style>

</style>
