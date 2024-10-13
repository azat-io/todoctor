<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
  import { toggleTheme, theme } from '~/stores/theme'
  import Logo from '~/elements/logo.svelte'
  import { data } from '~/stores/data'

  $theme
</script>

<header class="header">
  <Container>
    <div class="wrapper">
      <div>
        <div class="start-wrapper">
          <Logo size={72} />
          <div>
            <div class="title-wrapper">
              <Typography size="2xl" tag="h1" bold>Todoctor</Typography>
              {#if $data.version}
                <sub class="version">v{$data.version}</sub>
              {/if}
            </div>
            {#if $data.name}
              <Typography size="m">{$data.name}</Typography>
            {/if}
          </div>
        </div>
      </div>
      <div class="links">
        <button class="button" on:click={toggleTheme}>
          <Typography size="s" color="brand">Toggle Theme</Typography>
        </button>
        <a href="https://github.com/azat-io/todoctor" target="_blank">
          <Typography size="s" color="brand">GitHub</Typography>
        </a>
      </div>
    </div>
  </Container>
</header>

<style>
  .header {
    padding-block: var(--space-m);
  }

  .wrapper {
    display: flex;
    align-items: start;
    justify-content: space-between;
  }

  .start-wrapper {
    display: flex;
    gap: var(--space-2xs);
    align-items: center;
  }

  .title-wrapper {
    display: flex;
    gap: var(--space-s);
    align-items: start;
  }

  .version {
    margin-block-start: calc(var(--space-2xs) / 2);
    font: var(--font-xs);
  }

  .button {
    padding-inline: 0;
    color: var(--color-content-brand);
    text-decoration: underline;
    text-underline-offset: 0.25em;
    background: none;
    border: none;
    border-radius: var(--border-radius);
    outline: none;
    transition: box-shadow 200ms;

    &:focus-visible {
      text-decoration: none;
      background: var(--color-overlay-brand);
      box-shadow: 0 0 0 2px var(--color-border-brand);
    }
  }

  .links {
    display: flex;
    gap: var(--space-m);
  }
</style>
