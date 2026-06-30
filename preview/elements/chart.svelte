<script
  lang="ts"
  generics="T extends ChartType"
>
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
  export let data: ChartData
  export let options: ChartOptions
  export let height: number | null = null

  let canvasReference: HTMLCanvasElement
  let chart: Chart | null = null

  let resizeObserver: ResizeObserver | null = null
  let resizeFrame: number | null = null

  // Reapply `responsive`/`maintainAspectRatio` on every options change.
  // Assigning `chart.options` drops them and breaks resize/aspect handling.
  $: mergedOptions = {
    ...options,
    maintainAspectRatio: false,
    responsive: true,
  } as ChartOptions

  function createChart(): void {
    if (chart) {
      chart.destroy()
    }

    chart = new Chart(canvasReference, {
      options: mergedOptions,
      type,
      data,
    } satisfies ChartConfiguration)
  }

  onMount(() => {
    createChart()

    // Coalesce resize work into one `chart.resize()` per animation frame.
    // Deferring out of the observer callback prevents resize-time stutter.
    resizeObserver = new ResizeObserver(() => {
      if (resizeFrame !== null) {
        return
      }
      resizeFrame = requestAnimationFrame(() => {
        resizeFrame = null
        chart?.resize()
      })
    })

    if (canvasReference.parentElement) {
      resizeObserver.observe(canvasReference.parentElement)
    }
  })

  $: if (chart) {
    chart.data = data

    chart.options = mergedOptions
    chart.update()
  }

  onDestroy(() => {
    if (resizeFrame !== null) {
      cancelAnimationFrame(resizeFrame)
    }
    if (chart) {
      chart.destroy()
    }
    if (resizeObserver) {
      resizeObserver.disconnect()
    }
  })
</script>

<div class="canvas-wrapper">
  <canvas
    class="canvas"
    bind:this={canvasReference}
    {height}
  ></canvas>
</div>

<style>
  .canvas-wrapper {
    position: relative;
  }

  .canvas {
    display: block;
  }
</style>
