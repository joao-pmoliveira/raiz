<script lang="ts">
    import type { Paragraph } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { paragraph: Paragraph };

    let { paragraph } : Props = $props();
</script>

<p>{#each paragraph.children as block, index (index)}<ReaderRendered {block}/>{/each}</p>
