<script lang="ts">
    import Sidebar from '../components/Sidebar.svelte';

    let { children } = $props();
</script>

<div id="container">
    <Sidebar/>
    <main>
        {@render children()}
    </main>
</div>


<style>
    #container {
        display: grid;
        grid-template-columns: max-content 100%;
    }
</style>
