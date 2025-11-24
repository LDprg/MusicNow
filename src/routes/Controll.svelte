<script module>
    const play = true;
</script>

<div class="controll-container">
    <div class="icon">
        <i class="fa-solid fa-radiation"></i>
    </div>
    <div class="text">
        Song: abc
        <br />
        Artist: nc
        <br />
        Mbid: abc
    </div>
    <div class="ctrl">
        <div class="upper">
            <div class="center">
                {#if play}
                    <i class="fa-solid fa-play"></i>
                {:else}
                    <i class="fa-solid fa-pause"></i>
                {/if}
            </div>
            <div class="right">
                <label for="volume">Volume:</label>
                <input type="range" id="volume" min="0" max="100" value="50" />
            </div>
        </div>
        <input type="range" min="0" max="10020" value="5000" disabled />
    </div>
</div>

<style>
    .controll-container {
        background: var(--theme-bg-dark);
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

    .controll-container .ctrl .upper .right {
        align-self: center;
        flex: 1;
        font-size: 14px;
    }

    .controll-container .ctrl input[type="range"] {
        width: 100%;
    }
</style>
