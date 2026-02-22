<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'

  let currentYear = new Date().getFullYear()
</script>

<footer class="footer">
  <Container>
    <div class="wrapper">
      <div>
        <Typography size="m">Released under the MIT License</Typography>
        <Typography size="m">Copyright © {currentYear} Azat S.</Typography>
      </div>
      <a
        on:click={() => {
          if (globalThis.fathom) {
            globalThis.fathom.trackEvent('click: footer github')
          }
        }}
        href="https://github.com/azat-io/todoctor"
        rel="noopener noreferrer"
        target="_blank"
      >
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
    flex-wrap: nowrap;
    align-items: start;
    justify-content: space-between;
  }
</style>
