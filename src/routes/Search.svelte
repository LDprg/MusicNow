<script>
    import { invoke } from "@tauri-apps/api/core";

    let { searchResults = $bindable() } = $props();

    let query = $state("");

    /**
     * @param {Event} event
     */
    async function search(event) {
        event.preventDefault();

        console.info("Starting Search");

        searchResults = await invoke("search", {
            query: query,
            limit: 40,
            page: 1,
        });
    }

    $inspect(searchResults);
</script>

<form class="search-container" onsubmit={search}>
    <input type="text" placeholder="Search" bind:value={query} />
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button type="submit"><i class="fa fa-search"></i></button>
</form>

<style>
    .search-container {
        background: var(--theme-bg-darker);
        display: flex;
        position: sticky;
        top: 0;
        width: 100%;
    }

    .search-container input[type="text"] {
        padding: 6px;
        margin: 8px 0px 8px 8px;
        font-size: 18px;
        flex: 1;
    }

    .search-container button {
        float: right;
        padding: 6px 10px;
        margin: 8px;
        font-size: 18px;
    }
</style>
