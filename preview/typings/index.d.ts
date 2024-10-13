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
}

export interface Data {
  currentPath: string
  data: DataItem[]
  version: string
  name: string
}
