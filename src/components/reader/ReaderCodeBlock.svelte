<script lang="ts">
    import type { CodeBlockBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props {codeblock: CodeBlockBlock};

    let { codeblock } : Props = $props();
</script>

<div>
    <code id="tag">{codeblock.language || "no tag"}</code>
    <pre><code>{#each codeblock.lines as block, index (index)}<ReaderRendered {block}/>{/each}</code></pre>
</div>

<style>
    div {
        padding: 0.8rem 1rem;
        border-radius: 0.5em;
        background-color: hsl(210, 6%, 82%);
    }

    #tag {
        font-size: 0.75rem;
        font-style: italic;
    }
</style>
