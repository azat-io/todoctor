<script lang="ts">
  import type { ColumnDef } from '@tanstack/svelte-table'

  import { renderComponent } from '@tanstack/svelte-table'

  import TableComment from '~/elements/table-comment.svelte'
  import Typography from '~/elements/typography.svelte'
  import TableCode from '~/elements/table-code.svelte'
  import TableUser from '~/elements/table-user.svelte'
  import TablePath from '~/elements/table-path.svelte'
  import TableDate from '~/elements/table-date.svelte'
  import Container from '~/elements/container.svelte'
  import Table from '~/elements/table.svelte'
  import { data } from '~/stores/data'

  interface Column {
    comment: string
    summary: string
    author: string
    commit: string
    email: string
    path: string
    line: number
    date: string
  }

  $: preparedData =
    $data.data?.map(({ blame, ...rest }) => ({
      ...blame,
      ...rest,
    })) ?? []

  $: ({ currentPath } = $data)

  let columns: ColumnDef<Column>[] = [
    {
      cell: props =>
        renderComponent(TableUser, {
          author: props.getValue() as string,
          email: props.row.original.email,
        }),
      accessorKey: 'author',
      header: 'Author',
      size: 280,
    },
    {
      cell: props =>
        renderComponent(TablePath, {
          value: props.getValue() as string,
          line: props.row.original.line,
          currentPath,
        }),
      accessorKey: 'path',
      header: 'Path',
      size: 250,
    },
    {
      cell: props =>
        renderComponent(TableCode, {
          summary: props.row.original.summary,
          value: props.getValue() as string,
        }),
      accessorKey: 'commit',
      header: 'Commit',
      size: 80,
    },
    {
      cell: props =>
        renderComponent(TableComment, {
          value: props.getValue() as string,
        }),
      accessorKey: 'comment',
      header: 'Comment',
      size: 380,
    },
    {
      sortingFn: (a, b) =>
        new Date(a.original.date).getTime() -
        new Date(b.original.date).getTime(),
      cell: props =>
        renderComponent(TableDate, {
          value: props.getValue() as string,
        }),
      accessorKey: 'date',
      header: 'Added',
      size: 130,
    },
  ]
</script>

<div class="list">
  <Container>
    <Typography size="l" tag="h2" mbe="l">List of Commits</Typography>
    <Table data={preparedData} {columns} />
  </Container>
</div>

<style>
  .list {
    margin-block: var(--space-xl);
  }
</style>
