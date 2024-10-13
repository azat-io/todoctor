<script lang="ts">
  import Container from '~/elements/container.svelte'
  import Typography from '~/elements/typography.svelte'
</script>

<footer class="footer">
  <Container>
    <div class="wrapper">
      <div>
        <Typography size="xs">Released under the MIT License</Typography>
        <Typography size="xs">Copyright © 2024 Azat S.</Typography>
      </div>
      <a href="https://github.com/azat-io/todoctor" target="_blank">
        <Typography size="s">Source code on GitHub</Typography>
      </a>
    </div>
  </Container>
</footer>

<style>
  .footer {
    margin-block: var(--space-xl);
  }

  .wrapper {
    display: flex;
    align-items: start;
    justify-content: space-between;
  }
</style>
