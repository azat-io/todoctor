<script lang="ts">
  import Typography from '~/elements/typography.svelte'
  import Avatar from '~/elements/avatar.svelte'

  export let email = ''
  export let author = ''
</script>

<div class="user">
  <Avatar data={`${email}__${author}`} />
  <div class="info">
    <Typography
      size="m"
      noWrap>{author}</Typography
    >
    <a href="mailto:{email}">
      <Typography
        size="s"
        color="inherit"
        noWrap>{email}</Typography
      >
    </a>
  </div>
</div>

<style>
  .user {
    display: flex;
    gap: var(--space-xs);
    align-items: center;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 0;
    overflow: hidden;

    :global(p) {
      line-height: 1.4;
    }
  }
</style>
