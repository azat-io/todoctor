<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
</script>

<footer class="footer">
  <Container>
    <div class="wrapper">
      <div>
        <Typography size="m">Released under the MIT License</Typography>
        <Typography size="m">Copyright © 2024 Azat S.</Typography>
      </div>
      <a href="https://github.com/azat-io/todoctor" target="_blank">
        <Typography size="m">Source code on GitHub</Typography>
      </a>
    </div>
  </Container>
</footer>

<style>
  .footer {
    margin-block: var(--space-2xl) var(--space-xl);
  }

  .wrapper {
    display: flex;
    align-items: start;
    justify-content: space-between;
  }
</style>
