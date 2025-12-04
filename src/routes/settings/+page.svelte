<script>
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let global_volume = $state(50);
    let soundcloud = $state(false);
    let listenbrainz = $state(false);
    let lastfm = $state(false);

    function check() {
        invoke("is_soundcloud").then((data) => (soundcloud = data));
        invoke("is_listenbrainz").then((data) => (listenbrainz = data));
        invoke("is_lastfm").then((data) => (lastfm = data));
    }

    onMount(check);
</script>

<div class="container">
    <h1 style="color: var(--theme-warn)">
        Ui for reference only! Not fully working! WIP
    </h1>

    <div class="connections">
        <h2>Connections:</h2>
        <div class="grid">
            <button onclick={() => invoke("login_soundcloud").then(check)}>
                <i class="fa-brands fa-soundcloud"></i> Soundcloud
            </button>
            {#if soundcloud}
                <div class="status"><i class="fa-solid fa-check"></i></div>
            {:else}
                <div class="status"><i class="fa-solid fa-x"></i></div>
            {/if}

            <button onclick={() => invoke("login_listenbrainz").then(check)}>
                <i class="fa-solid fa-brain"></i> Listenbrainz
            </button>
            {#if listenbrainz}
                <div class="status"><i class="fa-solid fa-check"></i></div>
            {:else}
                <div class="status"><i class="fa-solid fa-x"></i></div>
            {/if}

            <button onclick={() => invoke("login_lastfm").then(check)}>
                <i class="fa-brands fa-lastfm"></i> LastFM
            </button>
            {#if lastfm}
                <div class="status"><i class="fa-solid fa-check"></i></div>
            {:else}
                <div class="status"><i class="fa-solid fa-x"></i></div>
            {/if}
        </div>
    </div>

    <br />

    <div class="settings">
        <h2>Settings:</h2>
        <label for="global_volume">
            Global Volume: {global_volume}% <br /></label
        >
        <input
            type="range"
            id="global_volume"
            min="0"
            max="100"
            step="1"
            bind:value={global_volume}
        />
    </div>
</div>

<style>
    .container {
        flex: 1;
        margin: 8px;
    }

    .connections button {
        background: var(--theme-bg-darker);
        font-size: 22px;
        margin-bottom: 8px;
        padding: 8px;
    }

    .connections button:hover {
        background: var(--theme-focus);
    }

    .connections .grid {
        display: grid;
        grid-template-columns: repeat(2, auto);
    }

    .connections .status {
        background: var(--theme-bg-dark);
        text-align: center;
        align-self: center;
        font-size: 22px;
        margin-left: 8px;
        margin-bottom: 8px;
        padding: 8px;
    }

    .connections .fa-check {
        color: var(--theme-succ);
    }

    .connections .fa-x {
        color: var(--theme-err);
    }

    .settings input[type="range"] {
        width: 100%;
    }
</style>
