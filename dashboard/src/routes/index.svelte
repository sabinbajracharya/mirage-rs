<script>
    let endpoints_promise = fetchEndpoints();

    async function fetchEndpoints() {
        const res = await fetch('http://localhost:8080/endpoints');
        const json = await res.json();

		if (res.ok) {
			return json;
		} else {
			throw new Error(json);
		}
	}
</script>

<h1>Api Endpoints</h1>

{#await endpoints_promise}
	<p>...waiting</p>
{:then items}
    <ul>
        {#each items as item (item.id)}
            <li><a href="#">{item.path}</a></li>
        {/each}
    </ul>
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

