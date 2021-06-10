<script >
    let is_loading = fetchEndpoints();
    let endpoints = [];
    let new_endpoint = '';

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

    async function edit(id) {

    }

    async function add() {
        new_endpoint = format(new_endpoint);

        if (new_endpoint.length === 0) {
            return;
        }
        const res = await fetch('http://localhost:8080/endpoint', {
            method: 'POST',
            headers: {
                'Content-type': 'application/json; charset=UTF-8'
            },
            body: JSON.stringify({
                "path": new_endpoint
            }),
        });
        if (res.status === 200) {
            await fetchEndpoints();
            new_endpoint = '';
        } else {
            try {
                const json = await res.json();
                alert(json.message);
            } catch {
                alert(`Error: ${res.status}`);
            }

        }
    }

    function format(path) {
        let new_path = path.replace(/\s/g, "");
        if (new_path.length > 0) {
            const beginsWithSlash = new_path[0] === "/";
            if (!beginsWithSlash) {
                new_path = `/${new_path}`;
            }

            if (new_path.length > 1) {
                const endsWithSlash = new_path[new_path.length - 1] === "/";
                if (endsWithSlash) {
                    new_path = new_path.slice(0, new_path.length - 1);
                }
            }
        }
        return new_path;
    }
</script>

<h2 class="header"> Mirage.rs </h2>

{#await is_loading}
    <p> ...loading </p>
{:then}
    <div class="content">
        <div class="toolbar">
            <form on:submit|preventDefault={add}>
                <input type="text" bind:value={new_endpoint} placeholder="/user/login">
                <button type="submit">Add</button>
            </form>
        </div>
        <table>
            <th>Endpoints</th>
            <th colspan="2">Actions</th>
            {#each endpoints as item(item.id)}
                <tr>
                    <td><span> <a href="#" on:click={remove}>{item.path}</a></span></td>
                    <td><span > <a href="#" on:click={() => edit(item.id)}>Edit</a></span></td>
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
    .toolbar {
        padding: 16px 0px;
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
            margin-top: 0px;
        }
    </style>
</svelte:head>