<script lang="ts" generics="T extends ChartType">
  /* global T */

  // eslint-disable-next-line no-unused-vars
  import type { ChartOptions, ChartData, ChartType } from 'chart.js'

  import { registerables, Chart } from 'chart.js'
  import { onDestroy, onMount } from 'svelte'

  Chart.register(...registerables)

  export let type: T
  export let data: ChartData<T>
  export let options: ChartOptions<T>
  export let height: number | null

  let props: Record<string, unknown> = {}

  if (height) {
    props = { height }
  }

  let canvasRef: HTMLCanvasElement

  export let chart: Chart<T> | null = null

  onMount(() => {
    chart = new Chart(canvasRef, {
      options: {
        ...options,
        animation: {
          duration: 1000,
        },
      },
      data,
      type,
    })
  })

  $: if (chart) {
    chart.data = data
    chart.options = {
      ...options,
      animation: {
        duration: 0,
      },
    }
    chart.update()
  }

  onDestroy(() => {
    if (chart) {
      chart.destroy()
      chart = null
    }
  })
</script>

<canvas bind:this={canvasRef} {...props}></canvas>
