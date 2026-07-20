<script lang="ts">
    import type { Emphasis } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { emphasis: Emphasis};

    let { emphasis }: Props = $props();
</script>

<em>
    {#each emphasis.children as block, index (index)}
        <ReaderRendered {block} />
    {/each}
</em>


<style>

</style>
