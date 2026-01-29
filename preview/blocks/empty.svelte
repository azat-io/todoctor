<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import Container from '~/elements/container.svelte'
</script>

<div class="wrapper">
  <Container>
    <div class="info">
      <Typography
        size="xl"
        tag="h2"
        mbe="l">No Todos Found</Typography
      >
      <Typography
        size="m"
        mbe="s"
      >
        Great job! It looks like there are currently no TODO comments in your
        project.
      </Typography>
      <Typography size="m">
        This could mean your code is clean and well-maintained, or perhaps it’s
        time to plan out the next steps and improvements.
      </Typography>
    </div>
  </Container>
</div>

<style>
  .wrapper {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    inline-size: 100%;
    block-size: 100%;
  }

  .info {
    max-inline-size: 460px;
  }
</style>
