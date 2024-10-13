<script lang="ts" generics="T extends Object">
  import type {
    TableOptions,
    SortingState,
    OnChangeFn,
    ColumnDef,
  } from '@tanstack/svelte-table'

  import { writable } from 'svelte/store'
  import {
    getSortedRowModel,
    getCoreRowModel,
    createTable,
    FlexRender,
  } from '@tanstack/svelte-table'

  export let data: T[] = []
  export let columns: ColumnDef<T>[] = []

  let sorting: SortingState = []

  let setSorting: OnChangeFn<SortingState> = updater => {
    if (updater instanceof Function) {
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
    data,
    columns,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    debugTable: true,
  })

  $: options.set({
    data,
    columns,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })

  $: table = createTable($options)
</script>

<div class="wrapper">
  <table class="table">
    <thead>
      {#each table.getHeaderGroups() as headerGroup}
        <tr class="tr">
          <th class="th" style="inline-size: 40px">#</th>
          {#each headerGroup.headers as header}
            <th
              class="th"
              style={header.getSize()
                ? `inline-size: ${header.getSize()}px`
                : ''}
            >
              {#if !header.isPlaceholder}
                <button
                  class="button"
                  class:cursor-pointer={header.column.getCanSort()}
                  class:select-none={header.column.getCanSort()}
                  on:click={header.column.getToggleSortingHandler()}
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
      {#each table.getRowModel().rows as row, index}
        <tr class="tr">
          <td class="td td-index" style="inline-size: 40px">{index + 1}</td>
          {#each row.getVisibleCells() as cell}
            <td style={`inline-size: ${cell.column.getSize()}px`} class="td">
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
    background-color: transparent;
    border: none;
    border-radius: var(--border-radius);
    outline: none;
    transition: box-shadow 200ms;

    &:focus-visible {
      background-color: var(--color-overlay-brand);
      box-shadow: 0 0 0 2px var(--color-border-brand);
    }
  }

  .th {
    padding: var(--space-s) var(--space-2xs);
    font: var(--font-s);
    font-weight: 700;
    text-align: start;
    background: var(--color-background-secondary);
  }

  .th,
  .td {
    border: 1px solid var(--color-border-primary);
  }

  .td {
    padding: var(--space-2xs);
    overflow-wrap: anywhere;
    vertical-align: top;
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
    background: color-mix(
      in lab,
      var(--color-background-primary),
      var(--color-background-secondary) 30%
    );
  }
</style>
