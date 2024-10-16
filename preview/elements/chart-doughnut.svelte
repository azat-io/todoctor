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

  let options: ChartOptions<'doughnut'> = {
    plugins: {
      legend: {
        display: false,
      },
    },
    responsive: true,
  }
</script>

<Chart type="doughnut" {options} {data} />
