<script>
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
    <!-- svelte-ignore a11y_consider_explicit_label -->
    <button onclick={() => (global.ui.menu = !global.ui.menu)} class="ctrl">
        <i class="fa fa-bars"></i>
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
            Playlist
        {/if}
    </button>
</nav>

{@render children()}

<style>
    nav {
        display: flex;
        flex-direction: column;
        float: left;
        height: 100%;
        width: 2em;
        font-size: 20px;
    }

    .big {
        width: 6em;
        text-align: left;
        font-size: 24px;
    }

    .ctrl {
        background: inherit;
        color: var(--theme-tertiary);
    }

    nav button {
        font-size: inherit;
        box-sizing: border-box;
        padding-bottom: 8px;
        padding-top: 8px;
    }
</style>
