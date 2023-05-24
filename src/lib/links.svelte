<script >
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/tauri"
    import { open } from '@tauri-apps/api/shell';
    import { isAddOpen } from "../stores/addLinkStore";
    import { links } from "../stores/linksStore";

    function openLink(url) {
        console.log(`Opening link: ${url}`);
        open(url);
    }

    async function refresh() {
        console.log("Refreshing links...");
        let linkss = await invoke("get_links_json")
        links.set(linkss); 
        console.log(`Links: ${JSON.stringify($links)}`);
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
        <button class="tool_bar_item" on:click={refresh}>Päivitä</button>
        <button class="tool_bar_item" on:click={() => {isAddOpen.set(true)}}>Lisää Linkki</button>
        <button class="tool_bar_item" on:click={quit}>Poistu</button>
    </div>
    
    <div class="links">
        {#each $links as link}
            <div class="link_item">
                <h1>{link.name}</h1>
                <button class="link_button" on:click={() => openLink(link.url)}>AVAA</button>
                <button class="link_delete" on:click={() => deleteLink(link.name)}>X</button>
            </div>
        {/each}
    </div>
</div>
