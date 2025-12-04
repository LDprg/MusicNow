<script>
    import Controll from "./Controll.svelte";

    import { goto } from "$app/navigation";
    import * as global from "$lib/global.svelte";

    let { children } = $props();

    let navStyle = $derived.by(() => {
        if (global.ui.menu) {
            return "big";
        }
        return "";
    });
</script>

<nav class={navStyle}>
    <div class="top">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button onclick={() => (global.ui.menu = !global.ui.menu)} class="ctrl">
            {#if global.ui.menu}
                <i class="fa fa-arrow-left"></i>
            {:else}
                <i class="fa fa-arrow-right"></i>
            {/if}
        </button>
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button onclick={() => goto("/")}>
            <i class="fa fa-home"></i>
            {#if global.ui.menu}
                Home
            {/if}
        </button>
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button onclick={() => goto("/playlist")}>
            <i class="fa fa-list"></i>
            {#if global.ui.menu}
                Playlists
            {/if}
        </button>
    </div>
    <div class="bottom">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button onclick={() => goto("/settings")}>
            <i class="fa fa-gear"></i>
            {#if global.ui.menu}
                Settings
            {/if}
        </button>
    </div>
</nav>

<main class="container">
    {@render children()}

    <Controll track={global.player.track} />
</main>

<style>
    .container {
        height: 100%;
        background: var(--theme-bg);
        display: flex;
        flex-direction: column;
    }

    nav {
        display: flex;
        flex-direction: column;
        float: left;
        height: 100%;
        width: 2.75em;

        font-size: 16px;
        @media (min-width: 720px) {
            font-size: 18px;
        }
        @media (min-width: 1080px) {
            font-size: 20px;
        }

        box-sizing: border-box;
        border: solid;
        border-width: 2px;
        border-color: var(--theme-bg-dark);

        .top {
            flex-direction: column;
            margin-top: 8px;
        }

        .bottom {
            flex-direction: column-reverse;
            margin-bottom: 8px;
        }
    }

    nav div {
        flex: 1;
        width: 100%;
        display: flex;
    }

    .big {
        width: 5em;
        text-align: left;

        font-size: 20px;
        @media (min-width: 720px) {
            font-size: 22px;
        }
        @media (min-width: 1080px) {
            font-size: 24px;
        }
    }

    .ctrl {
        background: inherit;
        color: var(--theme-tertiary);

        border-radius: 0;
        border-bottom: solid;
        border-width: 4px;
        border-color: var(--theme-bg);
    }

    nav button {
        font-size: inherit;
        box-sizing: border-box;
        border-radius: 8%;
        padding-bottom: 12px;
        padding-top: 12px;
        margin: 4px;
    }
</style>
