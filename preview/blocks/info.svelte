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
      let topFive = todosEntries.slice(0, 5)
      let remainingSum = todosEntries
        .slice(5)
        .reduce((sum, [, value]) => sum + value, 0)
      topFive.push(['OTHER', remainingSum])
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

<div class="wrapper">
  <Container>
    <div class="grid">
      <div>
        <Typography size="l" tag="h2" mbe="l">Did You Know That?</Typography>
      </div>
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
</div>

<style>
  .wrapper {
    margin-block: var(--space-2xl);
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr 420px;
    gap: var(--space-l);
  }

  .legend {
    display: flex;
    flex-wrap: wrap;
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
