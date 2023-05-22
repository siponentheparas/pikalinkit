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

    refresh();
</script>

<div>
    {#each links as link}
        <button on:click={() => openLink(link.url)}>{link.name}<br></button>
    {/each}
    <a href="riveria.fi" target="_blank" rel="noreferrer">riveira</a>
</div>
