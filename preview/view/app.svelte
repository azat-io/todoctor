<script lang="ts">
  import Header from '~/blocks/header.svelte'
  import Footer from '~/blocks/footer.svelte'
  import List from '~/blocks/list.svelte'
  import { data } from '~/stores/data'

  $: if ($data.name) {
    document.title = `${$data.name} | Todoctor`
  }
</script>

<Header />
<List />
<Footer />
