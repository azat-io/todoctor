<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let title = ''
  export let content: string[] = []

  interface Part {
    bold: boolean
    text: string
  }

  function processLine(line: string): Part[] {
    let parts: Part[] = []

    let regex = /<b>(?<content>.*?)<\/b>/giu

    let lastIndex = 0
    let match

    while ((match = regex.exec(line)) !== null) {
      if (match.index > lastIndex) {
        parts.push({ text: line.slice(lastIndex, match.index), bold: false })
      }
      parts.push({ text: match[1], bold: true })
      ;({ lastIndex } = regex)
    }

    if (lastIndex < line.length) {
      parts.push({ text: line.slice(lastIndex), bold: false })
    }

    return parts
  }
</script>

<div class="note">
  <Typography size="l" tag="h3" mbe="s">{title}</Typography>

  {#each content as line (line)}
    <Typography size="m" tag="p" mbe="2xs">
      {#each processLine(line) as part (part.text)}
        {#if part.bold}
          <b>{part.text}</b>
        {:else}
          {part.text}
        {/if}
      {/each}
    </Typography>
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
