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
    import Controll from "./Controll.svelte";

    /**
     * @type {any[string]}
     */
    let items = $state([]);

    [...Array(20).keys()].forEach((i) => items.push(i));
</script>

<main class="container">
    <Search />

    <div class="items-container">
        {#each items as item}
            <div class="items">
                <div class="icon">
                    <i class="fa fa-radiation"></i>
                </div>
                <table class="text">
                    <tbody>
                        <tr>
                            <th>Titel: </th>
                            <th>Nr. {item}</th>
                        </tr>
                        <tr>
                            <th>Artist: </th>
                            <th>Abcadaaaa</th>
                        </tr>
                        <tr>
                            <th>Mbid: </th>
                            <th>Abc</th>
                        </tr>
                    </tbody>
                </table>
            </div>
        {/each}
    </div>

    <Controll />
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
        display: flex;
        flex-flow: row wrap;
        align-content: flex-start;
    }

    .items {
        margin: 8px;
        display: flex;
        flex: 1;
        height: min-content;
    }

    .items .icon {
        align-self: center;
        font-size: 50px;
    }

    .items img {
        width: 50px;
        height: 50px;
    }

    .items .text {
        margin-left: 8px;
        font-size: 16px;
        text-align: left;
    }
</style>
