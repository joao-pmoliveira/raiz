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
        margin: 1.5rem 2rem;
        display: grid;
        grid-template-columns: max-content 1fr;
    }

    :global(body) {
        box-sizing: border-box;
        font-family:'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode', Geneva, Verdana, sans-serif;
        font-size: 14px;

        & li {
            padding: 0.5rem 0.8rem;
            margin: 0.5rem 0;
        }
    }
</style>
