<script lang="ts" generics="T extends ChartType">
  /* global T */

  import type {
    ChartConfiguration,
    ChartOptions,
    ChartData,
    ChartType,
  } from 'chart.js'

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
      } as ChartOptions,
      type: type as ChartType,
      data,
    } as unknown as ChartConfiguration<T>)
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
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
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
