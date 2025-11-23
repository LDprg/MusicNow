<script>
    import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

    /**
     * @param {'log' | 'debug' | 'info' | 'warn' | 'error'} fnName
     * @param {(message: string) => Promise<void>} logger
     */
    function forwardConsole(fnName, logger) {
        const original = console[fnName];
        console[fnName] = (message) => {
            original(message);
            logger(message);
        };
    }

    forwardConsole("log", trace);
    forwardConsole("debug", debug);
    forwardConsole("info", info);
    forwardConsole("warn", warn);
    forwardConsole("error", error);

    import Search from "./Search.svelte";

    /**
     * @type {any[string]}
     */
    let items = $state([]);

    [...Array(1000).keys()].forEach((i) => items.push(i));
</script>

<main class="container">
    <Search />

    <div class="items-container">
        {#each items as item}
            <div class="items">Item Nr: {item}</div>
        {/each}
    </div>

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
            <div>
                <i class="fa-solid fa-radiation"></i>
            </div>
            <input type="range" />
        </div>
    </div>
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
    }

    .items {
        margin-bottom: 2px;
    }

    .controll-container {
        background: var(--theme-bg-dark);
        width: 100%;
        bottom: 0;
        display: flex;
    }

    .controll-container .icon {
        font-size: 50px;
        margin: 8px;
    }

    .controll-container .text {
        margin: 8px;
    }

    .controll-container .ctrl {
        align-self: center;
        margin: 16px;
        flex: 1;
    }

    .controll-container .ctrl div {
        text-align: center;
        font-size: 18px;
        margin-bottom: 8px;
    }
    .controll-container .ctrl input[type="range"] {
        width: 100%;
        outline: none;
        cursor: pointer;
        accent-color: var(--theme-secondary);
    }

    .controll-container .ctrl input[type="range"]:hover {
        accent-color: var(--theme-primary);
    }
</style>
