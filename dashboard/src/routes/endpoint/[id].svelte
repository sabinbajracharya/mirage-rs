<script>
    import Breadcrumb from '../../components/breadcrumb.svelte';
    import { page } from '$app/stores';

    const navigationItems = [
        {
            "label": "Home",
            "path": "",
        },
        {
            "label": `Detail (id = ${$page.params.id})`,
            "path": "",
        }
    ];
</script>

<Breadcrumb items={navigationItems} />