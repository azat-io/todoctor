<script lang="ts">
  import { formatDistanceToNow, differenceInDays } from 'date-fns'

  import ChartDoughnut from '~/elements/chart-doughnut.svelte'
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
  import Note from '~/elements/note.svelte'
  import { data } from '~/stores/data'

  $: todosByKind =
    $data.data?.reduce((accumulator: Record<string, number>, { kind }) => {
      if (!kind) {
        return accumulator
      }

      accumulator[kind] = (accumulator[kind] ?? 0) + 1
      return accumulator
    }, {}) ?? {}

  $: todosEntries = Object.entries(todosByKind).toSorted(
    ([, a], [, b]) => b - a,
  )

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

  let notes: {
    content: string[]
    title: string
  }[]

  let pluralLabels: Record<Intl.LDMLPluralRule, string> = {
    other: 'todos',
    zero: 'todos',
    many: 'todos',
    two: 'todos',
    few: 'todos',
    one: 'todo',
  }

  function toTimestamp(value: string): number {
    let time = new Date(value).getTime()
    return Number.isNaN(time) ? Number.POSITIVE_INFINITY : time
  }

  function toDateOrNull(value: string): Date | null {
    let timestamp = toTimestamp(value)
    if (!Number.isFinite(timestamp)) {
      return null
    }
    return new Date(timestamp)
  }

  $: notes = (() => {
    let todos = $data.data
    if (!todos?.length) {
      return []
    }

    let { length } = todos

    let oldestRelative = (() => {
      let sortedTodos = todos.toSorted(
        (a, b) => toTimestamp(a.blame.date) - toTimestamp(b.blame.date),
      )

      let [oldestTodo] = sortedTodos
      if (!oldestTodo) {
        return '0 days'
      }

      let date = toDateOrNull(oldestTodo.blame.date)
      if (!date) {
        return '0 days'
      }

      return formatDistanceToNow(date)
    })()

    let dates = todos
      .map(todo => toDateOrNull(todo.blame.date))
      .filter((date): date is Date => date instanceof Date)

    let today = new Date()
    let totalDays = dates.reduce(
      (sum, date) => sum + differenceInDays(today, date),
      0,
    )
    let averageAge = dates.length > 0 ? Math.round(totalDays / dates.length) : 0

    let todosByAuthors = todos.reduce<Record<string, number>>(
      (accumulator, { blame }) => {
        let author = blame.author.trim()
        if (author.length === 0) {
          return accumulator
        }

        accumulator[author] = (accumulator[author] ?? 0) + 1
        return accumulator
      },
      {},
    )

    let authorEntries = Object.entries(todosByAuthors).toSorted(
      ([, a], [, b]) => b - a,
    )
    let [topAuthorEntry] = authorEntries
    let topAuthor = topAuthorEntry?.[0] ?? 'Unknown'
    let topAuthorTodos = topAuthorEntry?.[1] ?? 0
    let pluralForm = new Intl.PluralRules('en-US').select(topAuthorTodos)
    let todosFormatted = pluralLabels[pluralForm]

    return [
      {
        content: [
          `Currently, there are <b>${length}</b> todos in this project.`,
          "Tracking todos helps manage technical debt and ensures tasks aren't missed.",
        ],
        title: 'Number of Todos',
      },
      {
        content: [
          `The oldest todo was created <b>${oldestRelative} ago</b>.`,
          "It's a good opportunity to review and see if it's still relevant or can be resolved.",
        ],
        title: 'Oldest Todo',
      },
      {
        content: [
          `The average age of todos in this project is <b>${averageAge} days</b>.`,
          'Older todos highlight neglected code, offering a chance to fix long-standing issues.',
        ],
        title: 'Average Age of Todos',
      },
      {
        content: [
          `The author with the most todos is <b>${topAuthor}</b>, contributing <b>${topAuthorTodos}</b> ${todosFormatted}.`,
          'This contributor highlights areas for improvement.',
        ],
        title: 'Top Author of Todos',
      },
    ]
  })()
</script>

<div class="wrapper">
  <Container>
    <div class="grid">
      <div class="notes-wrapper">
        <Typography size="xl" tag="h2" mbe="l">Did You Know That?</Typography>
        <div class="notes">
          {#each notes as { content, title } (title)}
            <Note {title} {content} />
          {/each}
        </div>
      </div>
      <div class="chart">
        <Typography size="xl" tag="h2" mbe="l" align="center">
          Todos by Kind
        </Typography>
        {#if values.length}
          <ChartDoughnut {values} />
        {/if}
      </div>
    </div>
    <div class="legend">
      {#each finalTodosEntries as [kind], index (kind)}
        <div class="legend-element">
          <div
            style={`--color: var(--color-additional-${colors[index]});`}
            class="legend-color"
          ></div>
          <Typography size="s" tag="span">{kind.toUpperCase()}</Typography>
        </div>
      {/each}
    </div>
  </Container>
</div>

<style>
  :root {
    --size-chart: 460px;
  }

  .wrapper {
    margin-block: calc(var(--space-2xl) * 2) var(--space-l);
  }

  .grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-2xl);

    @container (inline-size >= 1280px) {
      grid-template-columns: 1fr var(--size-chart);
    }
  }

  .legend {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xl);
    justify-content: center;
    max-inline-size: 900px;
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

  .notes-wrapper {
    container-type: inline-size;
  }

  .notes {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-l);
    min-block-size: var(--size-chart);

    @container (inline-size >= 600px) {
      grid-template-columns: 1fr 1fr;
    }
  }

  .chart {
    display: flex;
    flex-direction: column;
    align-items: center;

    & :global(.canvas) {
      max-inline-size: calc(var(--size-chart) * 1.25);
      max-block-size: calc(var(--size-chart) * 1.25);
    }
  }
</style>
