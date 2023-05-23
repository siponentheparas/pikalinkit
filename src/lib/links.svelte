<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri"
    import { open } from '@tauri-apps/api/shell';

    let links = [];

    function openLink(url) {
        console.log(`Opening link: ${url}`);
        open(url);
    }

    async function refresh() {
        console.log("Refreshing links...");
        links = await invoke("get_links_json");
        console.log(`Links: ${JSON.stringify(links)}`);
    }

    async function quit(){
        await invoke("quit_app")
    }

    async function deleteLink(name) {
        await invoke('delete_link', { name: name })
        await refresh();
    }

    refresh();
</script>

<div>
    <div class="tool_bar">
        <button class="tool_bar_item">Lisää Linkki</button>
        <button class="tool_bar_item" on:click={quit}>Quittaa</button>
    </div>
    
    <div class="links">
        {#each links as link}
            <div class="link_item">
                <h1>{link.name}</h1>
                <button class="link_button" on:click={() => openLink(link.url)}>AVAA</button>
                <button class="link_delete" on:click={() => deleteLink(link.name)}>POISTA</button>
            </div>
        {/each}
    </div>
</div>
