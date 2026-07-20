<script lang="ts">

    import { resolve } from "$app/paths"

    interface Props {
        href: string,
        label: string
    }

    let {href, label}: Props = $props();

</script>

<a href={resolve(href, {})}>
    {label}
</a>
