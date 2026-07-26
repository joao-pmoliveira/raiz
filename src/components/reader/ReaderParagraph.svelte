<script lang="ts">
    import type { ParagraphBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { paragraph: ParagraphBlock };

    let { paragraph } : Props = $props();
</script>

<p>
    {#each paragraph.children as block, index (index)}
        <ReaderRendered {block}/>
    {/each}
</p>
