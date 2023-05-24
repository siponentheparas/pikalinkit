<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { links } from "../stores/linksStore";
    import { isAddOpen } from "../stores/addLinkStore";

    async function quit() {
        await invoke("quit_app");
    }

    async function refresh() {
        console.log("Refreshing links...");
        let linkss = await invoke("get_links_json");
        links.set(linkss);
        console.log(`Links: ${JSON.stringify($links)}`);
    }
</script>

<div class="tool_bar">
    <button class="tool_bar_item" on:click={refresh}>Päivitä</button>
    <button
        class="tool_bar_item"
        on:click={() => {
            isAddOpen.set(true);
        }}>Lisää Linkki</button
    >
    <button class="tool_bar_item" on:click={quit}>Poistu</button>
</div>

<style>
    .tool_bar {
        display: flex;
        background-color: #000000;
        justify-content: space-evenly;
        flex-wrap: nowrap;
        gap: 2rem;
        height: 3rem;
        align-items: center;
    }

    .tool_bar_item {
        height: 2rem;
        width: 6rem;
        background-color: rgb(26, 24, 24);
        color: rgb(255, 255, 255);
        border: 1px solid #ffffff;
        border-style: solid;
        border-radius: 5px;
        transition-duration: 0.1s;
    }

    .tool_bar_item:hover {
        background-color: #2e362f;
        color: white;
        box-shadow: 0 12px 16px 0 rgba(0, 0, 0, 0.24),
            0 17px 50px 0 rgba(0, 0, 0, 0.19);
    }

    .tool_bar_item:active {
        background-color: #407748;
        color: white;
    }
</style>
