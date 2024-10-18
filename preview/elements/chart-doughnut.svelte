<script lang="ts">
  import type { ChartOptions, ChartData } from 'chart.js'

  import Chart from '~/elements/chart.svelte'
  import { theme } from '~/stores/theme'

  export let values: number[]

  $: computedStyles = getComputedStyle(document.body)

  $: data = {
    datasets: [
      {
        backgroundColor: [
          computedStyles.getPropertyValue('--color-additional-primary'),
          computedStyles.getPropertyValue('--color-additional-secondary'),
          computedStyles.getPropertyValue('--color-additional-tertiary'),
          computedStyles.getPropertyValue('--color-additional-quaternary'),
          computedStyles.getPropertyValue('--color-additional-quinary'),
          computedStyles.getPropertyValue('--color-additional-senary'),
        ],
        borderColor: computedStyles.getPropertyValue(
          '--color-border-secondary',
        ),
        data: values,
      },
    ],
  } as ChartData<'doughnut'>

  theme.subscribe(() => {
    computedStyles = getComputedStyle(document.body)
  })

  $: options = {
    plugins: {
      tooltip: {
        callbacks: {
          labelColor: context => ({
            backgroundColor: (context.dataset.backgroundColor as string[])[
              context.dataIndex
            ],
            borderColor: computedStyles.getPropertyValue(
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
        borderColor: computedStyles.getPropertyValue('--color-border-primary'),
        titleColor: computedStyles.getPropertyValue('--color-content-primary'),
        bodyColor: computedStyles.getPropertyValue('--color-content-primary'),
        borderWidth: 1,
      },
      legend: {
        display: false,
      },
    },
    responsive: true,
  } as ChartOptions<'doughnut'>
</script>

<Chart type="doughnut" height={null} {options} {data} />
