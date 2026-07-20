<script lang="ts">
    import type { TableHead } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { tableHead: TableHead }

    let { tableHead }: Props = $props();

</script>

<thead>
    <tr>
    {#each tableHead.children as block, index (index)}
        <ReaderRendered {block}/>
    {/each}
    </tr>
</thead>


<style>

</style>
