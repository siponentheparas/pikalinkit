<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri"
    import { open } from '@tauri-apps/api/shell';

    let links = [];
    
    let showDelete = false;

    function openLink(url) {
        console.log(`Opening link: ${url}`);
        open(url);
    }

    
    function toggleDelete() {
        showDelete != showDelete;
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
        await invoke('deleteLink', { name: name })
    }

    refresh();
</script>

<div>
    <div class="tool_bar">
        <button class="tool_bar_item">Lisää Linkki</button>
        <button class="tool_bar_item" on:click={toggleDelete}>Poista Linkki</button>
        <button class="tool_bar_item" on:click={quit}>Quittaa</button>
    </div>
    
    <div class="links">
        {#each links as link}
            <div class="link_item">
                <h1>{link.name}</h1>
                <button class="link_button" on:click={() => openLink(link.url)}>AVAA</button>
            </div>
        {/each}
    </div>

    {#if showDelete}
        <div class="delete_popup">
            <input type="text" placeholder="Nimi" class="popup_input">
            <button class="popup_button_confirm">Poista</button>
            <button class="popup_button_cancel">Peruuta</button>
        </div>
    {/if}
</div>
