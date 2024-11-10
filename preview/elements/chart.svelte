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
  export let height: number | null = null

  let canvasReference: HTMLCanvasElement
  let chart: Chart<T> | null = null

  let resizeObserver: ResizeObserver | null = null

  let createChart = (): void => {
    if (chart) {
      chart.destroy()
    }

    chart = new Chart(canvasReference, {
      options: {
        ...options,
        maintainAspectRatio: false,
        responsive: true,
      },
      type,
      data,
    })
  }

  onMount(() => {
    createChart()

    resizeObserver = new ResizeObserver(() => {
      chart?.resize()
    })

    if (canvasReference.parentElement) {
      resizeObserver.observe(canvasReference.parentElement)
    }
  })

  $: if (chart) {
    chart.data = data
    chart.options = options
    chart.update()
  }

  onDestroy(() => {
    if (chart) {
      chart.destroy()
    }
    if (resizeObserver) {
      resizeObserver.disconnect()
    }
  })
</script>

<canvas class="canvas" bind:this={canvasReference} {height} />

<style>
  .canvas {
    inline-size: 100%;
    max-inline-size: 100%;
    block-size: auto;
  }
</style>
