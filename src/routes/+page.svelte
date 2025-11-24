<script>
    import Search from "./Search.svelte";
    import Controll from "./Controll.svelte";

    /**
     * @type {number[]}
     */
    let items = $state([]);

    [...Array(200).keys()].forEach((i) => items.push(i));
</script>

<main class="container">
    <Search />

    <div class="items-container">
        <div class="items">
            <div class="icon">
                <i class="fa fa-radiation"></i>
            </div>
            <div class="text">
                Titel: Nr. -1
                <br />
                Artist: Abcadaaaassssssssssssss skkkkkkkkkk
                <br />
                Mbid: Aba
            </div>
        </div>
        {#each items as item}
            <div class="items">
                <div class="icon">
                    <i class="fa fa-radiation"></i>
                </div>
                <div class="text">
                    Titel: Nr. {item}
                    <br />
                    Artist: Abcadaaaa
                    <br />
                    Mbid: Aba
                </div>
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

    .items .text {
        min-width: 200px;
        margin-left: 8px;
        font-size: 16px;
        text-align: left;
    }
</style>
