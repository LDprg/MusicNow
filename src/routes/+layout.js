// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;
export const prerender = true;

import "@fortawesome/fontawesome-free/css/all.min.css";
import "./global.css";

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
