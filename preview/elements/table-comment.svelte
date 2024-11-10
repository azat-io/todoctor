<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let value = ''
  export let kind = ''

  let highlightedParts: { highlighted: boolean; text: string }[]

  $: {
    let regex = new RegExp(`(${kind})`, 'i')
    highlightedParts = value.split(regex).map(part => ({
      highlighted: regex.test(part),
      text: part,
    }))
  }
</script>

<Typography size="m">
  {#each highlightedParts as part (part.text)}
    {#if part.highlighted}
      <i>{part.text}</i>
    {:else}
      {part.text}
    {/if}
  {/each}
</Typography>
