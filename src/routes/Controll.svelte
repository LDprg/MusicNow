<script>
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { Tween } from "svelte/motion";

    let { track } = $props();

    /**
     * @brief Time precision in ms
     */
    const timePrecision = 50;

    let play = $state(false);
    let volume = $state(0);
    let progress = new Tween(0, { delay: 0 });
    let duration = $state(Infinity);

    listen("volume", (payload) => {
        volume = Math.round(payload.payload * 10) / 10;
    });
    invoke("get_volume").then((vol) => (volume = Math.round(vol * 10) / 10));

    listen("play_state", (payload) => {
        let state = payload.payload;
        play = state.is_playing;
        duration = state.duration / timePrecision;
        progress.set(state.progress / timePrecision);
    });
    invoke("is_playing").then((state) => (play = state));

    setInterval(() => {
        if (play) {
            progress.target += 1;
        }
    }, timePrecision);

    /**
     * @param {Event} event
     */
    async function toggle_playback(event) {
        event.preventDefault();

        await invoke("toggle_play");
    }

    /**
     * @param {Event} event
     */
    async function set_volume(event) {
        event.preventDefault();

        await invoke("set_volume", { volume: volume });
    }

    $inspect(play);
</script>

<div class="container">
    <div class="controll-container">
        <div class="icon">
            <i class="fa-solid fa-radiation"></i>
        </div>
        <div class="text">
            {#if track == undefined}
                Song: -\-
                <br />
                Artist: -\-
            {:else}
                Song: {track.title}
                <br />
                Artist: {track.artist}
                <!-- <br /> -->
                <!-- Mbid: {track.mbid} -->
            {/if}
        </div>
        <div class="ctrl">
            <div class="center">
                <button onclick={toggle_playback}>
                    {#if play}
                        <i class="fa-solid fa-pause"></i>
                    {:else}
                        <i class="fa-solid fa-play"></i>
                    {/if}
                </button>
            </div>
            <div class="right">
                <label for="volume">Volume:</label>
                <input
                    type="range"
                    id="volume"
                    min="0"
                    step="0.1"
                    max="100"
                    bind:value={volume}
                    oninput={set_volume}
                />
            </div>
        </div>
    </div>
    <input
        type="range"
        min="0"
        max={duration}
        bind:value={progress.current}
        disabled
    />
</div>

<style>
    .container {
        background: var(--theme-bg-darker);
        width: 100%;
        bottom: 0;
        display: flex;
        flex-direction: column;

        box-sizing: border-box;
        padding: 8px;
    }

    .controll-container {
        width: 100%;
        display: flex;
    }

    .controll-container .icon {
        align-self: center;
        font-size: 50px;
        margin-right: 8px;
    }

    .controll-container .text {
        align-self: center;
        margin: 8px;
        width: 150px;
        @media (min-width: 720px) {
            width: 200px;
        }
        @media (min-width: 1080px) {
            width: 300px;
        }
    }

    .controll-container .ctrl {
        align-self: center;
        margin-left: 16px;
        flex: 1;
        display: flex;
    }

    .controll-container .ctrl .center {
        text-align: center;
        align-self: center;
        flex: 4;
    }

    .controll-container .ctrl .center button {
        background: none;
        font-size: 20px;
    }

    .controll-container .ctrl .right {
        align-self: center;
        flex: 1;
        font-size: 14px;
    }

    input[type="range"] {
        width: 100%;
        margin: 0;
    }
</style>
