<script lang="ts">
  import type { ChartOptions, ChartData } from 'chart.js'

  import { onMount } from 'svelte'

  import { createChartPlugins } from '~/elements/chart-plugins'
  import Chart from '~/elements/chart.svelte'
  import { theme } from '~/stores/theme'

  export let values: number[]
  export let labels: string[]

  let computedStyles: CSSStyleDeclaration | undefined
  let data: ChartData<'line'>
  let options: ChartOptions<'line'>

  let max = Math.max(...values)
  let min = Math.min(...values)

  let monthNames = [
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ]

  function formatLabel(index: number): string | null {
    let label = labels[index]
    if (!label) {
      return null
    }

    let date = new Date(label)
    if (Number.isNaN(date.getTime())) {
      return null
    }

    if (date.getDate() === 1) {
      return monthNames[date.getMonth()] ?? null
    }

    return null
  }

  function updateChartData(): void {
    if (!computedStyles) {
      return
    }

    data = {
      datasets: [
        {
          pointBackgroundColor: computedStyles.getPropertyValue(
            '--color-additional-primary',
          ),
          backgroundColor: computedStyles.getPropertyValue(
            '--color-additional-secondary',
          ),
          pointBorderColor: computedStyles.getPropertyValue(
            '--color-additional-primary',
          ),
          borderColor: computedStyles.getPropertyValue(
            '--color-additional-primary',
          ),
          borderJoinStyle: 'miter',
          pointBorderWidth: 3,
          borderWidth: 4,
          tension: 0.3,
          data: values,
          fill: false,
        },
      ],
      labels,
    } satisfies ChartData<'line'>
  }

  function updateStyles(): void {
    computedStyles = getComputedStyle(document.body)
    updateChartData()
  }

  $: options = {
    scales: {
      x: {
        ticks: {
          font: {
            family: computedStyles?.getPropertyValue('--font-family-base'),
            size: 16,
          },
          color: computedStyles?.getPropertyValue('--color-content-primary'),
          callback: (_value, index) => formatLabel(index),
          autoSkip: false,
          maxRotation: 0,
        },
        grid: {
          color: computedStyles?.getPropertyValue('--color-border-primary'),
        },
      },
      y: {
        ticks: {
          font: {
            family: computedStyles?.getPropertyValue('--font-family-base'),
            size: 16,
          },
          color: computedStyles?.getPropertyValue('--color-content-primary'),
        },
        grid: {
          color: computedStyles?.getPropertyValue('--color-border-primary'),
        },
        min: min === 0 ? 0 : min / 2,
        max: max + min / 2,
      },
    },
    plugins: createChartPlugins<'line'>(computedStyles, () =>
      computedStyles?.getPropertyValue('--color-additional-primary'),
    ),
  }

  onMount(() => {
    updateStyles()

    return theme.subscribe(() => {
      updateStyles()
    })
  })

  updateChartData()
</script>

<Chart
  type="line"
  height={null}
  {options}
  {data}
/>
