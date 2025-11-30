<script>
    import Search from "./Search.svelte";
    import Controll from "./Controll.svelte";

    import { invoke } from "@tauri-apps/api/core";

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

    /**
     * @param {Event} event
     */
    async function play(event) {
        event.preventDefault();

        if (track != undefined && track.mbid != null) {
            console.info("Playing song");

            await invoke("play", {
                trackId: Number.parseInt(track.mbid),
            });
        }
    }
</script>

<main class="container">
    <Search bind:searchResults={tracks} />

    <div class="items-container">
        {#each tracks as item}
            <button
                class="items"
                onclick={(event) => {
                    track = item;
                    play(event);
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

    <Controll {track} />
</main>

<style>
    .container {
        height: 100%;
        background: var(--theme-bg);
        display: flex;
        flex-direction: column;
    }

    .items-container {
        flex: 1;
        overflow: auto;
        display: grid;

        @media (min-width: 720px) {
            grid-template-columns: repeat(2, auto);
        }
        @media (min-width: 1080px) {
            grid-template-columns: repeat(3, auto);
        }
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
