import { readable, writable } from 'svelte/store'

import type { Data } from '~/typings/index.d'

let decodeHtmlEntities = (str: string): string => {
  let textArea = document.createElement('textarea')
  textArea.innerHTML = str
  return textArea.value
}

let traverseAndDecode = <T>(obj: T): T => {
  if (typeof obj === 'string') {
    return decodeHtmlEntities(obj) as T
  }

  if (Array.isArray(obj)) {
    return obj.map(item => traverseAndDecode(item)) as T
  }

  if (typeof obj === 'object' && obj !== null) {
    let result = {} as { [K in keyof T]: T[K] }
    for (let key in obj) {
      if (obj.hasOwnProperty(key)) {
        result[key] = traverseAndDecode(obj[key])
      }
    }
    return result
  }

  return obj
}

export let loading = writable(true)

export let data = readable<Partial<Data>>({}, set => {
  let fetchData = async () => {
    let dataValue: Data
    if (import.meta.env.MODE === 'production') {
      // @ts-ignore
      dataValue = window.data
    } else {
      let dataResponse = await fetch('/data.json')
      dataValue = await dataResponse.json()
    }
    set(traverseAndDecode(dataValue))
    loading.set(false)
  }

  fetchData()
})
