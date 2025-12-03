<script>
    import Search from "./Search.svelte";

    import * as global from "$lib/global.svelte";

    import { invoke } from "@tauri-apps/api/core";
    /**
     * @param {Event} event
     */
    async function play(event) {
        event.preventDefault();

        if (
            global.player.track != undefined &&
            global.player.track.mbid != null
        ) {
            console.info("Playing song");

            await invoke("play", {
                trackId: Number.parseInt(global.player.track.mbid),
            });
        }
    }
</script>

<Search bind:searchResults={global.player.tracks} />

<div class="items-container">
    {#each global.player.tracks as item}
        <div class="items">
            <button
                class="icon"
                onclick={(event) => {
                    global.player.track = item;
                    play(event);
                }}
            >
                {#if item.image_url == undefined}
                    <i class="fa fa-ban"></i>
                {:else}
                    <img
                        src={item.image_url}
                        alt={"Cover art of " +
                            item.title +
                            " from " +
                            item.artist}
                    />
                {/if}
            </button>
            <div class="text">
                <div class="title">{item.title}</div>
                <div>{item.artist}</div>
                <div>
                    ID:
                    {#if item.mbid == undefined}
                        <span class="warn">None</span>
                    {:else}
                        {item.mbid}
                    {/if}
                </div>
            </div>
        </div>
    {/each}
</div>

<style>
    .items-container {
        flex: 1;
        margin: 2px;
        margin-top: 4px;
        overflow: scroll;
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
        margin: 6px;
        padding: 6px;
        display: flex;
        flex: 1;
        background: var(--theme-bg-dark);
        color: var(--theme-fg);

        button {
            background: unset;
        }

        button:hover {
            background: unset;
        }
    }

    .items:hover {
        background: var(--theme-focus);
    }

    .items .icon {
        flex: 1;

        font-size: 80px;
        text-align: center;

        display: flex;
        align-self: center;
        align-items: center;

        min-width: 100px;
        max-width: 100px;
        min-height: 100px;
        max-height: 100px;

        * {
            flex: 1;
            width: 100%;
            height: 100%;
        }

        img {
            object-fit: cover;
        }
    }

    .items .text {
        flex: 1;
        margin-left: 8px;
        font-size: 16px;
        text-align: left;
        overflow-x: scroll;

        div {
            margin-bottom: 4px;
        }

        .title {
            font-size: 18px;
            font-weight: bold;
        }
    }

    .warn {
        color: var(--theme-warn);
    }
</style>
