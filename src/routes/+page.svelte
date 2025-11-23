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
        <i class="fa-solid fa-radiation"></i>
        Song
    </div>
</main>

<style>
    .container {
        font-size: 24;
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
        background-color: var(--theme-bg-dark);
        width: 100%;
        padding: 8px;
        box-sizing: border-box;
        position: sticky;
        bottom: 0;
    }
</style>
