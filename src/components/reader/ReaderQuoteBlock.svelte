<script lang="ts">
    import type { BlockQuote } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props {blockQuote: BlockQuote};

    let { blockQuote } : Props = $props();
    let blockQuoteType = $derived(blockQuote.blockquote_type);
</script>

<div>
<blockquote>
    {#if blockQuoteType !== "None"}
    <p id="tag">{blockQuoteType}</p>
    {/if}
    {#each blockQuote.children as block, index (index)}<ReaderRendered {block}/>{/each}
</blockquote>
</div>

<style>

</style>
