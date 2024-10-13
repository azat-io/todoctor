<script lang="ts">
  import type { ColumnDef } from '@tanstack/svelte-table'

  import TableComment from '~/elements/table-comment.svelte'
  import { renderComponent } from '@tanstack/svelte-table'
  import Container from '~/elements/container.svelte'
  import TableCode from '~/elements/table-code.svelte'
  import TableUser from '~/elements/table-user.svelte'
  import TablePath from '~/elements/table-path.svelte'
  import TableDate from '~/elements/table-date.svelte'
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

  $: currentPath = $data.currentPath

  let columns: ColumnDef<Column>[] = [
    {
      accessorKey: 'author',
      header: 'Author',
      cell: props =>
        renderComponent(TableUser, {
          author: props.getValue() as string,
          email: props.row.original.email,
        }),
      size: 280,
    },
    {
      accessorKey: 'path',
      header: 'Path',
      cell: props =>
        renderComponent(TablePath, {
          value: props.getValue() as string,
          line: props.row.original.line,
          currentPath: currentPath,
        }),
      size: 250,
    },
    {
      accessorKey: 'commit',
      header: 'Commit',
      cell: props =>
        renderComponent(TableCode, {
          summary: props.row.original.summary,
          value: props.getValue() as string,
        }),
      size: 80,
    },
    {
      accessorKey: 'comment',
      header: 'Comment',
      size: 380,
      cell: props =>
        renderComponent(TableComment, {
          value: props.getValue() as string,
        }),
    },
    {
      accessorKey: 'date',
      header: 'Added',
      cell: props =>
        renderComponent(TableDate, {
          value: props.getValue() as string,
        }),
      size: 130,
      sortingFn: (a, b) =>
        new Date(a.original.date).getTime() -
        new Date(b.original.date).getTime(),
    },
  ]
</script>

<div class="list">
  <Container>
    <Table data={preparedData} {columns} />
  </Container>
</div>

<style>
  .list {
    margin-block: var(--space-xl);
  }
</style>
