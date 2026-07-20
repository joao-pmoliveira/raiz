<script lang="ts">
    import type { ListItem } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { listItem: ListItem };

    let { listItem } : Props = $props();
</script>

<li>{#each listItem.children as block, index (index)}<ReaderRendered {block}/>{/each}</li>
