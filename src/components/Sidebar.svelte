<script>
    import NavItem from "./shared/NavItem.svelte";

</script>
<aside>
    <nav>
        <NavItem href="/library" label="Library"/>
        <NavItem href="/reader" label="Reader"/>
        <NavItem href="/settings" label="Settings"/>
    </nav>
</aside>

<style>
    nav {
        min-width: 7rem;
        display: flex;
        flex-direction: column;
    }
</style>
