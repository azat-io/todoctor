<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let value = ''
  export let kind = ''

  let formattedValue = value.replace(new RegExp(kind, 'i'), '<i>$&</i>')
</script>

<Typography size="m">
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  {@html formattedValue}
</Typography>
