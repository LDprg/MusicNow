<script>
    import Search from "./Search.svelte";

    import * as global from "$lib/global.svelte";

    import { invoke } from "@tauri-apps/api/core";

    // TODO: Replace by custom implementation at some point of time
    import InfiniteLoading from "svelte-infinite-loading";

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

    let page = $state(1); // page 0 is already loaded by search

    /**
     * @type {(() => void) | null}
     */
    let reset_fn = null;

    $effect(() => {
        if (global.player.tracks.length != 0) {
            if (reset_fn) reset_fn();
        }
    });

    // @ts-ignore
    function infiniteHandler({ detail: { loaded, complete, reset } }) {
        if (reset) reset_fn = reset;

        if (global.player.query != null && global.player.tracks != null) {
            invoke("search", {
                query: global.player.query,
                limit: 40,
                offset: page,
            }).then((data) => {
                if (data.length != 0) {
                    global.player.tracks = global.player.tracks.concat(data);
                    page += 1;
                    loaded();
                } else {
                    complete();
                }
            });
        } else {
            complete();
        }
    }
</script>

<Search bind:searchResults={global.player.tracks} />

<div class="items-container">
    {#each global.player.tracks as item, index}
        <div class="items" data-num={index + 1}>
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
                {#if item.available}
                    <div class="err">NOT AVAILABLE</div>
                {/if}
            </div>
        </div>
    {/each}

    <InfiniteLoading on:infinite={infiniteHandler}>
        <div slot="noResults"></div>
        <div slot="noMore"></div>
        <div slot="spinner">Loading...</div>
    </InfiniteLoading>
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

    .err {
        color: var(--theme-err);
    }
</style>
