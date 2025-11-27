<script>
    import Search from "./Search.svelte";
    import Controll from "./Controll.svelte";

    /**
     * @type {{title: String, artist: String, mbid: (null | String)}[]}
     */
    let items = $state([]);
</script>

<main class="container">
    <Search bind:searchResults={items} />

    <div class="items-container">
        {#each items as item}
            <div class="items">
                <div class="icon">
                    <i class="fa fa-radiation"></i>
                </div>
                <div class="text">
                    Titel: {item.title}
                    <br />
                    Artist: {item.artist}
                    <br />
                    Mbid: {item.mbid}
                </div>
            </div>
        {/each}
    </div>

    <Controll />
</main>

<style>
    .container {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .items-container {
        flex: 1;
        overflow: auto;
        display: flex;
        flex-flow: row wrap;
        align-content: flex-start;
    }

    .items {
        margin: 8px;
        display: flex;
        flex: 1;
        height: min-content;
    }

    .items .icon {
        align-self: center;
        font-size: 50px;
    }

    .items .text {
        min-width: 300px;
        margin-left: 8px;
        font-size: 16px;
        text-align: left;
    }
</style>
