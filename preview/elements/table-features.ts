import {
  sortFn_alphanumeric as sortFunctionAlphanumeric,
  sortFn_datetime as sortFunctionDatetime,
  sortFn_text as sortFunctionText,
  createSortedRowModel,
  columnSizingFeature,
  rowSortingFeature,
  tableFeatures,
} from '@tanstack/table-core'

export let features = tableFeatures({
  sortFns: {
    alphanumeric: sortFunctionAlphanumeric,
    datetime: sortFunctionDatetime,
    text: sortFunctionText,
  },
  sortedRowModel: createSortedRowModel(),
  columnSizingFeature,
  rowSortingFeature,
})

export type Features = typeof features
