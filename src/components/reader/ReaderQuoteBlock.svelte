<script lang="ts">
    import type { BlockQuoteBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { blockQuote: BlockQuoteBlock };

    let { blockQuote } : Props = $props();
    let blockQuoteType = $derived(blockQuote.kind);
</script>

<blockquote>
    {#if blockQuoteType !== "None"}
    <p id="tag">{blockQuoteType}</p>
    {/if}
    {#each blockQuote.children as block, index (index)}<ReaderRendered {block}/>{/each}
</blockquote>

<style>
    blockquote {
        margin: 0;
        padding: 0.8rem 1rem;
        border-radius: 0.5rem ;
        background-color: hsl(210, 6%, 82%);
    }

    #tag {
        font-size: 0.75rem;
        font-style: italic;
    }
</style>
