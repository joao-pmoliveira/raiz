<script lang="ts">
    import type { Strong } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { strong: Strong};

    let { strong }: Props = $props();
</script>

<strong>
    {#each strong.children as block, index (index)}
        <ReaderRendered {block} />
    {/each}
</strong>


<style>

</style>
