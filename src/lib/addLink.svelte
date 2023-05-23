<script>
    // @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri";
    import { isAddOpen } from "../stores/addLinkStore";
    import { links } from "../stores/linksStore";

    async function refresh() {
        console.log("Refreshing links...");
        links.set(await invoke("get_links_json"));
        console.log(`Links: ${JSON.stringify($links)}`);
    }

    async function addLink(name, url) {
        console.log("link name length: " + name.length);
        if (name != "" && url != "" && name.length <= 13) {
            await invoke("add_link", { name: name, url: url });
            isAddOpen.set(false);
            refresh();
        }
    }

    let name = "";
    let url = "";
</script>

<div class="add_link_bar">
    <input
        type="text"
        placeholder="Nimi"
        class="add_link_input"
        bind:value={name}
    />
    <input
        type="text"
        placeholder="URL"
        class="add_link_input"
        bind:value={url}
    />
    <button class="add_link_button_add" on:click={addLink(name, url)}
        >Lisää</button
    >
    <button
        class="add_link_button_cancel"
        on:click={() => {
            isAddOpen.set(false);
        }}>Peruuta</button
    >
</div>

<style>
    .add_link_bar {
        background-color: #000000;
        display: flex;
        border-bottom: 0.5px solid white;
        height: 4rem;
        justify-content: space-evenly;
        align-items: center;
        padding-bottom: 1rem;
    }

    .add_link_input {
        background-color: black;
        border: 1px solid #ffffff;
        border-radius: 15px;
        color: #ffffff;
        font-size: 20px;
        text-align: center;
    }

    .add_link_button_add {
        height: 3rem;
        width: 5rem;
        background-color: rgb(26, 24, 24);
        color: rgb(255, 255, 255);
        border: 2px solid #ffffff;
        border-radius: 25px;
        transition-duration: 0.1s;
    }

    .add_link_button_add:hover {
        background-color: rgb(2, 102, 20);
    }

    .add_link_button_add:active {
        background-color: rgb(0, 165, 55);
    }

    .add_link_button_cancel {
        height: 3rem;
        width: 5rem;
        background-color: rgb(54, 6, 6);
        color: rgb(255, 255, 255);
        border: 2px solid #ffffff;
        border-radius: 25px;
        transition-duration: 0.1s;
    }

    .add_link_button_cancel:hover {
        background-color: rgb(145, 0, 0);
    }

    .add_link_button_cancel:active {
        background-color: rgb(255, 0, 0);
    }
</style>
