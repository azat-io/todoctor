import {
  sortFn_alphanumeric as sortFunctionAlphanumeric,
  sortFn_datetime as sortFunctionDatetime,
  tableFeatures as createTableFeatures,
  sortFn_text as sortFunctionText,
  createSortedRowModel,
  columnSizingFeature,
  rowSortingFeature,
} from '@tanstack/table-core'

export let tableFeatures = createTableFeatures({
  sortFns: {
    alphanumeric: sortFunctionAlphanumeric,
    datetime: sortFunctionDatetime,
    text: sortFunctionText,
  },
  sortedRowModel: createSortedRowModel(),
  columnSizingFeature,
  rowSortingFeature,
})

export type TableFeatures = typeof tableFeatures
