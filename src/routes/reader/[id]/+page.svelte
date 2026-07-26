<script lang="ts">

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import type { Resource } from "../../library/library";
    import type { PageProps } from './$types';
    import ReaderRendered from "../../../components/reader/ReaderRendered.svelte";

    let { params }: PageProps = $props();
    let resource: Resource | null = $state(null);

    onMount(async () => {
        resource = await invoke<Resource>("get_resource", {uuid: params.id});
        console.log(resource)
    })

</script>

{#if resource}
    <div>
        <h2>{resource.metadata.title}</h2>
        <p>{resource.metadata.path}</p>
        <p>{resource.metadata.resource_type}</p>
    </div>
    <hr>
    {#if resource.document?.blocks}
        {#each resource.document.blocks as block, index (index)}
            <ReaderRendered {block} />
        {/each}
    {/if}

{/if}


