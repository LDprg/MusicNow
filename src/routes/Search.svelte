<script>
</script>

<form class="search-container">
    <input type="text" placeholder="Search" />
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button type="submit"><i class="fa fa-search"></i></button>
</form>

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
        background: var(--theme-bg);
        color: var(--theme-fg);
        border: none;
        flex: 1;
    }

    .search-container button {
        float: right;
        padding: 6px 10px;
        margin: 8px;
        background: var(--theme-bg);
        color: var(--theme-primary);
        font-size: 18px;
        border: none;
        cursor: pointer;
    }

    .search-container button:hover {
        background: var(--theme-focus);
    }
</style>
