<script >
    let is_loading = fetchEndpoints();
    let endpoints = [];

    async function fetchEndpoints() {
        const res = await fetch('http://localhost:8080/endpoints');
        const json = await res.json();

        if (res.ok) {
            endpoints = json;
            return json;
        } else {
            throw new Error(json);
        }
    }

    async function remove(id) {
        const res = await fetch(`http://localhost:8080/endpoint/${id}`, {
            method: 'DELETE',
        });
        if (res.status === 200) {
            endpoints = [...endpoints.slice(0, endpoints.length - 1)];
        }
    }
</script>

<h2 class="header"> Mirage.rs </h2>

{#await is_loading}
    <p> ...loading </p>
{:then}
    <div class="content">
    <table>
        <th>Endpoints</th>
        <th></th>
        {#each endpoints as item(item.id)}
            <tr>
                <td><span> <a href = "#" on:click={remove}>{item.path}</a></span></td>
                <td><span > <a href="#" on:click={() => remove(item.id)}>Delete</a></span></td>
            </tr>
        {/each}
    </table>
    </div>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}

<style >
    .header {
        padding: 8px;
        background-color: slategrey;
        font-family: 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        color: white;
    }
    .content {
        font-family: "Roboto";
        max-width: 1024px;
        margin: auto;
        background: white;
        padding: 10px;
    }
    table {
        font-family: arial, sans-serif;
        border-collapse: collapse;
        width: 100%;
    }

    td, th {
        border: 1px solid #eeeeee;
        text-align: left;
        padding: 8px;
    }

    tr:nth-child(even) {
        background-color: #eeeeee;
    }
</style>

<svelte:head>
    <style>
        body {
            margin: 0px;
            padding: 0px;
        }

        h1, h2 {
            margin-top: 0 px;
        }
    </style>
</svelte:head>