<script lang="ts">
  import { onMount } from 'svelte'

  import { loading, data } from '~/stores/data'
  import Header from '~/blocks/header.svelte'
  import Footer from '~/blocks/footer.svelte'
  import Graph from '~/blocks/graph.svelte'
  import Empty from '~/blocks/empty.svelte'
  import Info from '~/blocks/info.svelte'
  import List from '~/blocks/list.svelte'

  $: length = $data.data?.length ?? 0

  onMount(() => {
    if ($data.name) {
      document.title = `${$data.name} | Todoctor`
    }
  })
</script>

<Header />
<main>
  {#if !$loading}
    {#if length}
      <Graph />
      <Info />
      <List />
    {:else}
      <Empty />
    {/if}
  {/if}
</main>
<Footer />
