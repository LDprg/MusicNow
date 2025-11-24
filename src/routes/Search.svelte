<script module>
    import { invoke } from "@tauri-apps/api/core";

    let name = $state("");
    let greetMsg = $state("Search for Message");

    /**
     * @param {Event} event
     */
    async function greet(event) {
        event.preventDefault();
        greetMsg = await invoke("greet", { name });
    }
</script>

<form class="search-container" onsubmit={greet}>
    <input type="text" placeholder="Search" bind:value={name} />
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button type="submit"><i class="fa fa-search"></i></button>
</form>
<!-- Temporary Code -->
<p>{greetMsg}</p>

<style>
    .search-container {
        background: var(--theme-bg-dark);
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
