<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let value = ''
  export let currentPath = ''
  export let line = 0
</script>

<a href={`${currentPath}/${value}`}>
  <Typography size="m">{`${value}:${line}`}</Typography>
</a>
