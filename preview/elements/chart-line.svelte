<script lang="ts">
  import type { ChartOptions, ChartData } from 'chart.js'

  import { onMount } from 'svelte'

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
    } as ChartData<'line'>

    options = {
      plugins: {
        tooltip: {
          callbacks: {
            labelColor: () => ({
              backgroundColor: computedStyles?.getPropertyValue(
                '--color-additional-primary',
              ),
              borderColor: computedStyles?.getPropertyValue(
                '--color-border-primary',
              ),
              borderRadius: 2,
              borderWidth: 0,
            }),
          },
          titleFont: {
            family: computedStyles.getPropertyValue('--font-family-base'),
            size: 16,
          },
          bodyFont: {
            family: computedStyles.getPropertyValue('--font-family-base'),
            size: 16,
          },
          backgroundColor: computedStyles.getPropertyValue(
            '--color-background-secondary',
          ),
          footerFont: {
            family: computedStyles.getPropertyValue('--font-family-base'),
          },
          borderColor: computedStyles.getPropertyValue(
            '--color-border-primary',
          ),
          titleColor: computedStyles.getPropertyValue(
            '--color-content-primary',
          ),
          bodyColor: computedStyles.getPropertyValue('--color-content-primary'),
          borderWidth: 1,
        },
        legend: {
          display: false,
        },
      },
      scales: {
        x: {
          ticks: {
            font: {
              family: computedStyles.getPropertyValue('--font-family-base'),
              size: 16,
            },
            color: computedStyles.getPropertyValue('--color-content-primary'),
            callback: (_value, index) => formatLabel(index),
            autoSkip: false,
            maxRotation: 0,
          },
          grid: {
            color: computedStyles.getPropertyValue('--color-border-primary'),
          },
        },
        y: {
          ticks: {
            font: {
              family: computedStyles.getPropertyValue('--font-family-base'),
              size: 16,
            },
            color: computedStyles.getPropertyValue('--color-content-primary'),
          },
          grid: {
            color: computedStyles.getPropertyValue('--color-border-primary'),
          },
          min: min === 0 ? 0 : min / 2,
          max: max + min / 2,
        },
      },
    } as ChartOptions<'line'>
  }

  function updateStyles(): void {
    computedStyles = getComputedStyle(document.body)
    updateChartData()
  }

  $: options = {
    plugins: {
      tooltip: {
        callbacks: {
          labelColor: () => ({
            backgroundColor: computedStyles?.getPropertyValue(
              '--color-additional-primary',
            ),
            borderColor: computedStyles?.getPropertyValue(
              '--color-border-primary',
            ),
            borderRadius: 2,
            borderWidth: 0,
          }),
        },
        titleFont: {
          family: computedStyles?.getPropertyValue('--font-family-base'),
          size: 16,
        },
        bodyFont: {
          family: computedStyles?.getPropertyValue('--font-family-base'),
          size: 16,
        },
        backgroundColor: computedStyles?.getPropertyValue(
          '--color-background-secondary',
        ),
        footerFont: {
          family: computedStyles?.getPropertyValue('--font-family-base'),
        },
        borderColor: computedStyles?.getPropertyValue('--color-border-primary'),
        titleColor: computedStyles?.getPropertyValue('--color-content-primary'),
        bodyColor: computedStyles?.getPropertyValue('--color-content-primary'),
        borderWidth: 1,
      },
      legend: {
        display: false,
      },
    },
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
  } as ChartOptions<'line'>

  onMount(() => {
    updateStyles()

    return theme.subscribe(() => {
      updateStyles()
    })
  })

  updateChartData()
</script>

<Chart type="line" height={null} {options} {data} />
