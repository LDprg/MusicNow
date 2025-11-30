<script>
    import { invoke } from "@tauri-apps/api/core";

    let { track } = $props();

    let play = $state(true);
    let volume = $state(50);

    /**
     * @type {Promise<void> | null}
     * @brief Syncs date with backend. Ensures only one running sync at the time
     * @todo This is very bad switch to two way implemenation
     */
    let syncFn = null;
    async function syncData() {
        if (syncFn) return syncFn;

        syncFn = (async () => {
            try {
                let vol = await invoke("get_volume");
                volume = Math.round(vol * 10) / 10; // Round to prevent unnessary reloading

                play = await invoke("is_playing");
            } finally {
                syncFn = null;
            }
        })();

        return syncFn;
    }

    syncData();

    /**
     * @param {Event} event
     */
    async function toggle_playback(event) {
        event.preventDefault();

        await invoke("toggle_play");
        await syncData();
    }

    /**
     * @param {Event} event
     */
    async function set_volume(event) {
        event.preventDefault();

        await invoke("set_volume", { volume: volume });
        await syncData();
    }

    $inspect(volume);
    $inspect(play);
</script>

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
        <div class="upper">
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
        <input type="range" min="0" max="10020" value="5000" disabled />
    </div>
</div>

<style>
    .controll-container {
        background: var(--theme-bg-darker);
        width: 100%;
        bottom: 0;
        display: flex;
    }

    .controll-container .icon {
        align-self: center;
        font-size: 50px;
        margin: 8px;
    }

    .controll-container .text {
        align-self: center;
        margin: 8px;
        width: 200px;
    }

    .controll-container .ctrl {
        align-self: center;
        margin: 16px;
        flex: 1;
    }

    .controll-container .ctrl .upper {
        text-align: center;
        font-size: 18px;
        margin-bottom: 8px;
        display: flex;
    }

    .controll-container .ctrl .upper .center {
        align-self: center;
        flex: 4;
    }

    .controll-container .ctrl .upper .center button {
        background: none;
        font-size: 20px;
    }

    .controll-container .ctrl .upper .right {
        align-self: center;
        flex: 1;
        font-size: 14px;
    }

    .controll-container .ctrl input[type="range"] {
        width: 100%;
    }
</style>
