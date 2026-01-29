<script
  generics="T extends object"
  lang="ts"
>
  import type {
    TableOptions,
    SortingState,
    OnChangeFn,
    ColumnDef,
  } from '@tanstack/svelte-table'

  import { getSortedRowModel, getCoreRowModel } from '@tanstack/table-core'
  import { createTable, FlexRender } from '@tanstack/svelte-table'
  import { writable } from 'svelte/store'

  export let data: T[] = []
  export let columns: ColumnDef<T>[] = []

  let sorting: SortingState = []

  let setSorting: OnChangeFn<SortingState> = updater => {
    if (typeof updater === 'function') {
      sorting = updater(sorting)
    } else {
      sorting = updater
    }
    options.update(old => ({
      ...old,
      state: {
        ...old.state,
        sorting,
      },
    }))
  }

  let options = writable<TableOptions<T>>({
    getSortedRowModel: getSortedRowModel(),
    getCoreRowModel: getCoreRowModel(),
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    debugTable: true,
    columns,
    data,
  })

  $: options.set({
    getSortedRowModel: getSortedRowModel(),
    getCoreRowModel: getCoreRowModel(),
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    columns,
    data,
  })

  $: table = createTable($options)
</script>

<div class="wrapper">
  <table class="table">
    <thead>
      {#each table.getHeaderGroups() as headerGroup, headerGroupIndex (headerGroupIndex)}
        <tr class="tr">
          <th
            style:inline-size="40px"
            class="th">#</th
          >
          {#each headerGroup.headers as header, headerIndex (headerIndex)}
            <th
              style={header.getSize() ?
                `inline-size: ${header.getSize()}px`
              : ''}
              class="th"
            >
              {#if !header.isPlaceholder}
                <button
                  on:click={event => {
                    let handler = header.column.getToggleSortingHandler()
                    handler?.(event)

                    if (globalThis.fathom) {
                      let columnName =
                        typeof header.column.columnDef.header === 'string' ?
                          header.column.columnDef.header.toLowerCase()
                        : 'unknown'
                      globalThis.fathom.trackEvent(
                        `table: sort by ${columnName}`,
                      )
                    }
                  }}
                  class:cursor-pointer={header.column.getCanSort()}
                  class:select-none={header.column.getCanSort()}
                  class="button"
                  type="button"
                >
                  <FlexRender
                    content={header.column.columnDef.header}
                    context={header.getContext()}
                  />
                  {#if header.column.getIsSorted().toString() === 'asc'}
                    ▲
                  {:else if header.column.getIsSorted().toString() === 'desc'}
                    ▼
                  {/if}
                </button>
              {/if}
            </th>
          {/each}
        </tr>
      {/each}
    </thead>
    <tbody>
      {#each table.getRowModel().rows as row, rowIndex (rowIndex)}
        <tr class="tr">
          <td
            style:inline-size="40px"
            class="td td-index">{rowIndex + 1}</td
          >
          {#each row.getVisibleCells() as cell, cellIndex (cellIndex)}
            <td
              style={`inline-size: ${cell.column.getSize()}px`}
              class="td"
            >
              <FlexRender
                content={cell.column.columnDef.cell}
                context={cell.getContext()}
              />
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .wrapper {
    overflow: auto;
    border-radius: var(--border-radius);
    box-shadow: 0 0 0 1px var(--color-border-primary);
  }

  .table {
    inline-size: 100%;
    min-inline-size: 800px;
    overflow: clip;
    table-layout: fixed;
    border-collapse: collapse;
    border-style: hidden;
  }

  .cursor-pointer {
    cursor: pointer;
  }

  .select-none {
    user-select: none;
  }

  .button {
    padding: 0;
    font: inherit;
    text-align: start;
    cursor: pointer;
    outline: none;
    background-color: transparent;
    border: none;
    border-radius: var(--border-radius);
    transition: box-shadow 200ms;

    &:focus-visible {
      background-color: var(--color-overlay-brand);
      box-shadow: 0 0 0 2px var(--color-border-brand);
    }
  }

  .th {
    padding: var(--space-s) var(--space-2xs);
    font: var(--font-m);
    font-weight: bold;
    text-align: start;
    background: var(--color-background-secondary);
  }

  .th,
  .td {
    border: 1px solid var(--color-border-primary);
  }

  .td {
    padding: var(--space-2xs);
    vertical-align: top;
    overflow-wrap: anywhere;
  }

  .td-index {
    white-space: nowrap;
  }

  .tr .th:first-child {
    border-radius: 4px 0 0 4px;
  }

  tr:nth-child(odd) {
    background: var(--color-background-primary);
  }

  tr:nth-child(even) {
    background: var(--color-background-tertiary);
  }
</style>
