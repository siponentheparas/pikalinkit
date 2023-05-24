<script>
    // @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/shell";
    import { links } from "../stores/linksStore";

    function openLink(url) {
        console.log(`Opening link: ${url}`);
        open(url);
    }

    async function refresh() {
        console.log("Refreshing links...");
        let linkss = await invoke("get_links_json");
        links.set(linkss);
        console.log(`Links: ${JSON.stringify($links)}`);
    }

    async function deleteLink(name) {
        await invoke("delete_link", { name: name });
        await refresh();
    }

    refresh();
</script>

<div>
    <div class="links">
        {#each $links as link}
            <div class="link_item">
                <h1>{link.name}</h1>
                <button class="link_button" on:click={() => openLink(link.url)}
                    >AVAA</button
                >
                <button
                    class="link_delete"
                    on:click={() => deleteLink(link.name)}>X</button
                >
            </div>
        {/each}
    </div>
</div>

<style>
    .link_item {
        display: flex;
        width: 20rem;
        height: 7rem;
        background-color: rgb(59, 53, 59);
        justify-content: space-evenly;
        align-items: center;
        border-radius: 10px;
    }

    .link_button {
        height: 3rem;
        width: 5rem;
        background-color: rgb(26, 24, 24);
        color: rgb(255, 255, 255);
        border: 2px solid #ffffff;
        border-radius: 25px;
        transition-duration: 0.1s;
    }

    .link_delete {
        height: 1.5rem;
        width: 1.5rem;
        background-color: rgb(26, 24, 24);
        color: rgb(255, 255, 255);
        border: 2px solid #ffffff;
        border-radius: 25px;
        text-align: center;
        transition-duration: 0.1s;
    }

    .link_delete:hover {
        background-color: #740000;
        color: white;
        box-shadow: 0 12px 16px 0 rgba(0, 0, 0, 0.24),
            0 17px 50px 0 rgba(0, 0, 0, 0.19);
    }

    .link_delete:active {
        background-color: #ac0000;
        color: white;
    }

    .link_button:hover {
        background-color: #2e362f;
        color: white;
        box-shadow: 0 12px 16px 0 rgba(0, 0, 0, 0.24),
            0 17px 50px 0 rgba(0, 0, 0, 0.19);
    }

    .link_button:active {
        background-color: #407748;
        color: white;
    }

    .links {
        display: flex;
        margin: 3rem;
        justify-content: start;
        flex-wrap: wrap;
        gap: 1rem;
    }
</style>
