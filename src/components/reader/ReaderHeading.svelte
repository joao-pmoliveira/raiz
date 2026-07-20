<script lang="ts">
    import type { Heading } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { heading: Heading };

    let { heading } : Props = $props();

    const heading_level = {
        1: "h1",
        2: "h2",
        3: "h3",
        4: "h4",
        5: "h5",
        6: "h6",
    }

</script>

<svelte:element this={heading_level[heading.level]}>
    {#each heading.children as block, index (index)}
        <ReaderRendered {block}/>
    {/each}
</svelte:element>

<style>

</style>
