export interface DataItem {
  blame: {
    summary: string
    author: string
    commit: string
    email: string
    date: string
  }
  comment: string
  line: number
  path: string
  kind: string
}

export interface Data {
  history: HistoryItem[]
  currentPath: string
  data: DataItem[]
  version?: string
  name: string
}

export interface HistoryItem {
  count: number
  date: string
}
