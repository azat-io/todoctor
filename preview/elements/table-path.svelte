<script lang="ts">
  import Typography from '~/elements/typography.svelte'

  export let value = ''
  export let currentPath = ''
  export let line = 0

  console.log('currentPath', currentPath)
</script>

<a href={`${currentPath}/${value}`}>
  <Typography size="s">{`${value}:${line}`}</Typography>
</a>
