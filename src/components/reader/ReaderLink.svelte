<script lang="ts">
    import { resolve } from "$app/paths";
    import type { Link } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";


    interface Props {
        link: Link
    }

    let { link }: Props = $props();

    const external = link.destination_url.startsWith("http://") || link.destination_url.startsWith("https://");;

</script>

{#if external}
<a href={link.destination_url} target="_blank" rel="noopener noreferrer">
    {#each link.children as block, index (index)}<ReaderRendered {block}/>{/each}
</a>
{:else}
<a href={resolve(link.destination_url, {})}>
    {#each link.children as block, index (index)}<ReaderRendered {block}/>{/each}
</a>
{/if}
