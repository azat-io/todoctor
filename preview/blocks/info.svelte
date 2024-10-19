<script lang="ts">
  import { formatDistanceToNow, differenceInDays } from 'date-fns'

  import ChartDoughnut from '~/elements/chart-doughnut.svelte'
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
  import Note from '~/elements/note.svelte'
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

  $: notes = [] as {
    content: string[]
    title: string
  }[]

  $: if ($data.data) {
    let { length } = $data.data

    let oldestDate = new Date(
      $data.data.toSorted(
        (a, b) =>
          new Date(a.blame.date).getTime() - new Date(b.blame.date).getTime(),
      )[0].blame.date,
    )
    let oldestRelative = formatDistanceToNow(oldestDate)

    let dates = $data.data.map(todo => new Date(todo.blame.date))
    let today = new Date()
    let totalDays = dates.reduce(
      (sum, date) => sum + differenceInDays(today, date),
      0,
    )
    let averageAge = Math.round(totalDays / dates.length)

    let todosByAuthors = $data.data.reduce(
      (accumulator: Record<string, number>, { blame }) => {
        if (blame.author in accumulator) {
          accumulator[blame.author] = accumulator[blame.author] + 1
        } else {
          accumulator[blame.author] = 1
        }
        return accumulator
      },
      {},
    )
    let [[topAuthor, topAuthorTodos]] = Object.entries(todosByAuthors).toSorted(
      ([, a], [, b]) => b - a,
    )
    let todosFormatted = {
      other: 'todos',
      many: 'todos',
      zero: 'todos',
      few: 'todos',
      two: 'todos',
      one: 'todo',
    }[new Intl.PluralRules('en-US').select(topAuthorTodos)]

    notes = [
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
  }
</script>

<div class="wrapper">
  <Container>
    <div class="grid">
      <div>
        <Typography size="xl" tag="h2" mbe="l">Did You Know That?</Typography>
        <div class="notes">
          {#each notes as { content, title }}
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

  .notes {
    display: grid;
    grid-template-rows: 1fr 1fr;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-l);
    min-block-size: var(--size-chart);
  }
</style>
