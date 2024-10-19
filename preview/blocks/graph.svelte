<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import ChartLine from '~/elements/chart-line.svelte'
  import Container from '~/elements/container.svelte'
  import { data } from '~/stores/data'

  $: history =
    $data.history?.sort(
      (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime(),
    ) ?? []
  $: labels = history?.map(({ date }) => date)
  $: values = history?.map(({ count }) => count)
</script>

<div class="graph-wrapper">
  <Container>
    <Typography size="xl" tag="h2" mbe="l">Todos Graph</Typography>
    <div>
      {#if values.length && labels.length}
        <ChartLine {values} {labels} />
      {/if}
    </div>
  </Container>
</div>

<style>
  .graph-wrapper {
    margin-block: var(--space-xl) var(--space-2xl);
  }
</style>
