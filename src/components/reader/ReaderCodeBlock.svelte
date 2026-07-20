<script lang="ts">
    import type { CodeBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props {codeblock: CodeBlock};

    let { codeblock } : Props = $props();
</script>

<div>
    <p id="tag">{codeblock.codeblock_type}</p>
    <pre><code>{#each codeblock.children as block, index (index)}<ReaderRendered {block}/>{/each}</code>
    </pre>
</div>

<style>

</style>
