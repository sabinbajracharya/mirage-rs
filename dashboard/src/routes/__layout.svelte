<script>
    // import { base } from '$app/paths';
    // import { breadcrumb } from '../store.js';
</script>

<h2 class="header"> Mirage.rs </h2>

<slot></slot>

<style>
    .header {
        padding: 8px;
        background-color: slategrey;
        font-family: 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        color: white;
    }
</style>

<svelte:head>
    <style>
        body {
            margin: 0px;
            padding: 0px;
        }

        h1, h2 {
            margin-top: 0px;
        }

        .content {
            font-family: "Roboto";
            max-width: 1024px;
            margin: auto;
            background: white;
            padding: 10px;
        }
    </style>
</svelte:head>