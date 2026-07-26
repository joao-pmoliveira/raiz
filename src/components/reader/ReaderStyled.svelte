<script lang="ts">
    import { StyledKind, type StyledBlock } from "../../routes/library/library";
    import ReaderRendered from "./ReaderRendered.svelte";

    interface Props { styled: StyledBlock};

    let { styled }: Props = $props();

    const styleComputed = {
        [StyledKind.Emphasis] : "em",
        [StyledKind.Strikethrough] : "del",
        [StyledKind.Strong] : "strong",
        [StyledKind.Subscript] : "sub",
        [StyledKind.Superscript] : "sup",
    }
</script>


<svelte:element this={styleComputed[styled.style]}>
    {#each styled.children as block, index (index)}
        <ReaderRendered {block} />
    {/each}
</svelte:element>


<style>

</style>
