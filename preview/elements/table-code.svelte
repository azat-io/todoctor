<script lang="ts">
  import { Tooltip } from '@svelte-plugins/tooltips'

  export let value = ''
  export let summary = ''
</script>

<Tooltip content={summary} theme="custom-tooltip">
  <code class="code">
    {value}
  </code>
</Tooltip>

<style>
  .code {
    margin: 0;
    font-size: var(--font-size-2xs);
    color: var(--color-content-brand);
    text-decoration: underline;
    cursor: help;
  }

  :global(.tooltip.custom-tooltip) {
    --tooltip-background-color: var(--color-background-secondary);
    --tooltip-box-shadow: 0 0 1px var(--color-border-primary);
    --tooltip-font-family: var(--font-family-base);
    --tooltip-color: var(--color-content-primary);
    --tooltip-border-radius: var(--border-radius);
    --tooltip-font-size: var(--font-size-xs);
    --tooltip-line-height: 1.35rem;
  }
</style>
