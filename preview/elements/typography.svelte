<script lang="ts">
  export let color: 'secondary' | 'primary' | 'inherit' | 'brand' = 'inherit'
  export let size: '2xl' | '2xs' | 'xl' | 'xs' | 'l' | 'm' | 's' = 'm'
  export let mbs: '2xs' | 'xs' | 'xl' | null | 's' | 'm' | 'l' = null
  export let mbe: '2xs' | 'xs' | 'xl' | null | 's' | 'm' | 'l' = null
  export let align: 'center' | 'right' | 'left' = 'left'
  export let noWrap: boolean = false
  export let bold: boolean = false
  export let tag = 'p'
</script>

<svelte:element
  this={tag}
  class={`typography size-${size} color-${color} weight-${bold ? 'bold' : 'normal'} ${mbs ? `mbs-${mbs}` : ''}${mbe ? `mbe-${mbe}` : ''} align-${align} ${noWrap ? 'no-wrap' : ''}`}
>
  <slot />
</svelte:element>

<style>
  .typography {
    margin-block: 0;
  }

  .size-2xs {
    font: var(--font-2xs);
  }

  .size-xs {
    font: var(--font-xs);
  }

  .size-s {
    font: var(--font-s);
  }

  .size-m {
    font: var(--font-m);
  }

  .size-l {
    font: var(--font-l);
  }

  .size-xl {
    font: var(--font-xl);
  }

  .size-2xl {
    font: var(--font-2xl);
  }

  .color-brand {
    color: var(--color-content-brand);
  }

  .color-primary {
    color: var(--color-content-primary);
  }

  .color-secondary {
    color: var(--color-content-secondary);
  }

  .color-inherit {
    color: inherit;
  }

  .weight-bold {
    font-weight: 600;
  }

  .weight-normal {
    font-weight: 400;
  }

  .mbs-2xs {
    margin-block-start: var(--space-2xs);
  }

  .mbs-xs {
    margin-block-start: var(--space-xs);
  }

  .mbs-s {
    margin-block-start: var(--space-s);
  }

  .mbs-m {
    margin-block-start: var(--space-m);
  }

  .mbs-l {
    margin-block-start: var(--space-l);
  }

  .mbs-xl {
    margin-block-start: var(--space-xl);
  }

  .mbe-2xs {
    margin-block-end: var(--space-2xs);
  }

  .mbe-xs {
    margin-block-end: var(--space-xs);
  }

  .mbe-s {
    margin-block-end: var(--space-s);
  }

  .mbe-m {
    margin-block-end: var(--space-m);
  }

  .mbe-l {
    margin-block-end: var(--space-l);
  }

  .mbe-xl {
    margin-block-end: var(--space-xl);
  }

  .align-start {
    text-align: start;
  }

  .align-center {
    text-align: center;
  }

  .align-end {
    text-align: end;
  }

  .no-wrap {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
