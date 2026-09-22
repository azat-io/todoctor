<script lang="ts">
  import type { ChartOptions, ChartData } from 'chart.js'

  import { onMount } from 'svelte'

  import { createChartPlugins } from '~/elements/chart-plugins'
  import Chart from '~/elements/chart.svelte'
  import { theme } from '~/stores/theme'

  export let values: undefined | number[]

  let computedStyles: CSSStyleDeclaration | undefined
  let data: ChartData<'doughnut'>
  let options: ChartOptions<'doughnut'>

  function updateChartData(): void {
    if (!computedStyles || !values) {
      return
    }

    data = {
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
    } satisfies ChartData<'doughnut'>
  }

  function updateStyles(): void {
    computedStyles = getComputedStyle(document.body)
    updateChartData()
  }

  $: options = {
    plugins: createChartPlugins<'doughnut'>(
      computedStyles,
      item => (item.dataset.backgroundColor as string[])[item.dataIndex],
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
  type="doughnut"
  height={null}
  {options}
  {data}
/>
