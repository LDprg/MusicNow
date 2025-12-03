import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

/**
 * @param {'log' | 'debug' | 'info' | 'warn' | 'error'} fnName
 * @param {(message: string) => Promise<void>} logger
 */
function forwardConsole(fnName, logger) {
    const original = console[fnName];
    console[fnName] = (message) => {
        original(message);
        logger(message.toString());
    };
}

forwardConsole("log", trace); // WARN: Problems with svelte $inspect
forwardConsole("debug", debug);
forwardConsole("info", info);
forwardConsole("warn", warn);
forwardConsole("error", error);
