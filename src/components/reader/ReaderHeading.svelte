<script lang="ts">
    import type { HeadingBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { heading: HeadingBlock };

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
    h1 {
        font-size: 1.8rem;
    }
    h2 {
        font-size: 1.6rem;
    }
    h3 {
        font-size: 1.4rem;
    }
    h4 {
        font-size: 1.2rem;
    }
    h5 {
        font-size: 1.1rem;
    }
    h6 {
        font-size: 1rem;
    }
</style>
