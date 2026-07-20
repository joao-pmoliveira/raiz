<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { type Library, type Resource } from "./library";
    import { onMount } from "svelte";
    import { resolve } from "$app/paths";

    let resources : Resource[] = $state([])

    onMount(get_library)

    async function get_resource() {
        await invoke<Resource>("select_resource");
        get_library();
    }

    async function get_library() {
        const lib = await invoke<Library>("get_library");
        resources = lib.resources
    }

</script>

<div>
    <h1>Library</h1>
    <p>You don't have any content yet.</p>
    <button onclick={() => get_resource()}>Import Book</button>
    <ul>
        {#each resources as resource (resource.metadata.title)}
            <li>
                <a href={resolve("/reader/[id]", {id: resource.metadata.id})}>{resource.metadata.title}</a>
            </li>
        {/each}
    </ul>
</div>
