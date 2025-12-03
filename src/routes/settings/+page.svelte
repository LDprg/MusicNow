<script>
    let global_volume = $state(50);
</script>

<div class="container">
    <h1 style="color: var(--theme-warn)">
        Ui for reference only! Not working! WIP
    </h1>

    <div class="connections">
        <h2>Connections:</h2>
        <div class="grid">
            <button>
                <i class="fa-brands fa-soundcloud"></i> Soundcloud
            </button>
            <div class="status"><i class="fa-solid fa-check"></i></div>

            <button> <i class="fa-solid fa-brain"></i> Listenbrainz </button>
            <div class="status"><i class="fa-solid fa-x"></i></div>

            <button> <i class="fa-brands fa-lastfm"></i> LastFM </button>
            <div class="status"><i class="fa-solid fa-x"></i></div>
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
