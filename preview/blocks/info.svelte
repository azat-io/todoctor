<script lang="ts">
  import ChartDoughnut from '~/elements/chart-doughnut.svelte'
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
  import { data } from '~/stores/data'

  $: todosByKind =
    $data.data?.reduce((accumulator: Record<string, number>, { kind }) => {
      if (kind in accumulator) {
        accumulator[kind] = accumulator[kind] + 1
      } else {
        accumulator[kind] = 1
      }
      return accumulator
    }, {}) || {}

  $: todosEntries = Object.entries(todosByKind).sort(([, a], [, b]) => b - a)

  $: finalTodosEntries = (() => {
    if (todosEntries.length > 5) {
      let topFive = todosEntries.slice(0, 4)
      let remainingSum = todosEntries
        .slice(4)
        .reduce((sum, [, value]) => sum + value, 0)
      topFive.push(['TOTAL', remainingSum])
      return topFive
    }
    return todosEntries
  })()

  $: values = finalTodosEntries.map(([, value]) => value)

  let colors = [
    'primary',
    'secondary',
    'tertiary',
    'quaternary',
    'quinary',
    'senary',
  ]
</script>

<Container>
  <div class="wrapper">
    <div></div>
    <div class="chart">
      <Typography size="l" tag="h2" mbe="l" align="center">
        Todos by Kind
      </Typography>
      {#if values.length}
        <ChartDoughnut {values} />
      {/if}
    </div>
  </div>
  <div class="legend">
    {#each finalTodosEntries as [kind], index}
      <div class="legend-element">
        <div
          style={`--color: var(--color-additional-${colors[index]});`}
          class="legend-color"
        ></div>
        <Typography size="s" tag="span">{kind}</Typography>
      </div>
    {/each}
  </div>
</Container>

<style>
  .wrapper {
    display: grid;
    grid-template-columns: 1fr 420px;
    gap: var(--space-l);
    margin-block-end: var(--space-xl);
  }

  .legend {
    display: flex;
    gap: var(--space-xl);
    justify-content: center;
    max-inline-size: 800px;
    margin-block-start: var(--space-xl);
    margin-inline: auto;
  }

  .legend-element {
    display: flex;
    gap: var(--space-s);
    align-items: center;
  }

  .legend-color {
    inline-size: var(--space-l);
    block-size: var(--space-l);
    background: var(--color);
    border-radius: var(--border-radius);
  }
</style>
