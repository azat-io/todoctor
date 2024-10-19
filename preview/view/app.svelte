<script lang="ts">
  import { loading, data } from '~/stores/data'
  import Header from '~/blocks/header.svelte'
  import Footer from '~/blocks/footer.svelte'
  import Graph from '~/blocks/graph.svelte'
  import Empty from '~/blocks/empty.svelte'
  import Info from '~/blocks/info.svelte'
  import List from '~/blocks/list.svelte'

  $: if ($data.name) {
    document.title = `${$data.name} | Todoctor`
  }
</script>

<Header />
<main>
  {#if !$loading}
    {#if $data.data?.length}
      <Graph />
      <Info />
      <List />
    {:else}
      <Empty />
    {/if}
  {/if}
</main>
<Footer />
