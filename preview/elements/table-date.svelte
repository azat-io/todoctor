<script lang="ts">
  import { formatDistanceToNow } from 'date-fns'

  import Typography from './typography.svelte'

  export let value = new Date().toString()

  $: relativeValue = formatDistanceToNow(new Date(value))

  $: formattedValue = new Intl.DateTimeFormat('en').format(new Date(value))
</script>

<div class="date">
  <Typography size="m">
    {`${relativeValue} ago`}
  </Typography>
  <Typography color="secondary" size="s">
    {formattedValue}
  </Typography>
</div>

<style>
  .date {
    display: flex;
    flex-direction: column;
    gap: 0;
    overflow: hidden;

    :global(p) {
      line-height: 1.4;
    }
  }
</style>
