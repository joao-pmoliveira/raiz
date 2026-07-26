<script lang="ts">
    import { resolve } from "$app/paths";
    import type { LinkBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";


    interface Props { link: LinkBlock }

    let { link }: Props = $props();

    const external = $derived(
        link.destination.startsWith("http://") || link.destination.startsWith("https://")
    );


</script>

{#if external}
<a href={link.destination} target="_blank" rel="noopener noreferrer">
    {#each link.children as block, index (index)}<ReaderRendered {block}/>{/each}
</a>
{:else}
<a href={resolve(link.destination, {})}>
    {#each link.children as block, index (index)}<ReaderRendered {block}/>{/each}
</a>
{/if}
