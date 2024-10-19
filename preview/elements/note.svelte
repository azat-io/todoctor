<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let title = ''
  export let content: string[] = []
</script>

<div class="note">
  <Typography size="l" tag="h3" mbe="s">{title}</Typography>
  {#each content as line}
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    <Typography size="m" tag="p" mbe="2xs">{@html line}</Typography>
  {/each}
</div>

<style>
  .note {
    padding: var(--space-l);
    background: var(--color-background-tertiary);
    border: 1px solid var(--color-border-primary);
    border-radius: var(--border-radius);
  }
</style>
