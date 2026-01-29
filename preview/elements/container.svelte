<script lang="ts">
  export let tag = 'div'
</script>

<svelte:element
  this={tag}
  class="container"
>
  <slot />
</svelte:element>

<style>
  .container {
    inline-size: 100%;
    max-inline-size: 1680px;
    padding: 0 var(--space-m);
    margin-inline: auto;
    container-type: inline-size;
  }
</style>
