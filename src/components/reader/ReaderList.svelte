<script lang="ts">
    import { type ListBlock } from "../../routes/library/library";
    import ReaderListItem from "./ReaderListItem.svelte";

    interface Props { list: ListBlock };

    let { list } : Props = $props();
</script>

{#if "Ordered" in list.kind}
    <ol>
        {#each list.items as block, index(index)}
            <ReaderListItem listItem={block} />
        {/each}
    </ol>
{:else if "Unordered" in list.kind }
    <ul>
        {#each list.items as block, index(index)}
            <ReaderListItem listItem={block} />
        {/each}
    </ul>
{/if}
