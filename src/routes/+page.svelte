<script>
    import Search from "./Search.svelte";
    import Controll from "./Controll.svelte";

    /**
     * @typedef {{title: String, artist: String, mbid: (null | String)}} Track
     */

    /**
     * @type {Track[]}
     */
    let tracks = $state([]);

    /**
     * @type {Track | undefined}
     */
    let track = $state();
</script>

<main class="container">
    <Search bind:searchResults={tracks} />

    <div class="items-container">
        {#each tracks as item}
            <button
                class="items"
                onclick={() => {
                    track = item;
                }}
            >
                <div class="icon">
                    <i class="fa fa-radiation"></i>
                </div>
                <div class="text">
                    Titel: {item.title}
                    <br />
                    Artist: {item.artist}
                    <br />
                    <br />
                    {#if item.mbid == undefined}
                        <span class="warn">None</span>
                    {:else}
                        {item.mbid}
                    {/if}
                </div>
            </button>
        {/each}
    </div>

    <Controll bind:track />
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
        display: grid;
        grid-template-columns: repeat(3, auto);
    }

    .items {
        box-sizing: border-box;
        margin: 4px;
        padding: 8px;
        display: flex;
        flex: 1;
        background: var(--theme-bg-dark);
        color: var(--theme-fg);
    }

    .items .icon {
        align-self: center;
        font-size: 50px;
    }

    .items .text {
        margin-left: 8px;
        font-size: 16px;
        text-align: left;
    }

    .warn {
        color: var(--theme-warn);
    }
</style>
