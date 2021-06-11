<script>
    import { base } from '$app/paths';

    export let items = [];
</script>

<div class="content">
    <ul>
        {#each items as item, i }
            {#if i == items.length -1}
                <li>{item.label}</li>
            {:else}
                <li><a href="{base}/{item.path}">{item.label}</a></li>
                <li> / </li>
            {/if}
        {/each}
    </ul>
</div>

<style>
    ul li{
        display: inline;
    }
</style>