<script lang="ts">
  import { Tooltip } from '@svelte-plugins/tooltips'

  export let value = ''
  export let summary = ''
</script>

<Tooltip
  theme="custom-tooltip"
  content={summary}
>
  <code class="code">
    {value}
  </code>
</Tooltip>

<style>
  .code {
    display: block;
    margin: calc(var(--space-2xs) + 2px) 0;
    font-family: Courier, monospace;
    font-size: var(--font-size-m);
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
    --tooltip-font-size: var(--font-size-m);
    --tooltip-line-height: 1.75rem;
  }
</style>
