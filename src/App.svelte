<script>
  import AddLink from './lib/addLink.svelte';
  import Links from './lib/links.svelte'
  import { isAddOpen } from './stores/addLinkStore';
</script>

{#if $isAddOpen}
  <AddLink />
{/if}

<main>
  <div>
    <Links />
  </div>
</main>
