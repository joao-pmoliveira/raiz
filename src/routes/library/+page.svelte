<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { ResourceMetadata, Resource } from "./library";
    import { onMount } from "svelte";
    import { resolve } from "$app/paths";

    let resources : ResourceMetadata[] = $state([])

    onMount(get_library)

    async function select_resource() {
        await invoke<Resource>("select_resource");
        get_library();
    }

    async function get_library() {
        try {
            resources = await invoke<ResourceMetadata[]>("get_library");
        } catch (err) {
            console.log(err);
        }
    }

</script>

<div>
    <h1>Library</h1>
    <p>You don't have any content yet.</p>
    <button onclick={() => select_resource()}>Import Book</button>
    <ul>
        {#each resources as resource (resource.title)}
            <li>
                <a href={resolve("/reader/[id]", {id: resource.uuid})}>{resource.title}</a>
            </li>
        {/each}
    </ul>
</div>

<style>
</style>
