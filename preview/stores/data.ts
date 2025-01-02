import { readable, writable } from 'svelte/store'

import type { Data } from '~/typings/index.d'

let decodeHtmlEntities = (string: string): string => {
  let textArea = document.createElement('textarea')
  textArea.innerHTML = string
  return textArea.value
}

let traverseAndDecode = <T>(object: T): T => {
  if (typeof object === 'string') {
    return decodeHtmlEntities(object) as T
  }

  if (Array.isArray(object)) {
    return object.map(item => traverseAndDecode(item) as T) as T
  }

  if (typeof object === 'object' && object !== null) {
    let result = {} as { [K in keyof T]: T[K] }
    for (let key in object) {
      if (key in object) {
        result[key] = traverseAndDecode(object[key])
      }
    }
    return result
  }

  return object
}

export let loading = writable(true)

export let data = readable<Partial<Data>>({}, set => {
  let fetchData = async (): Promise<void> => {
    let dataValue: Data
    if (import.meta.env.MODE === 'production') {
      dataValue = JSON.parse(
        document.querySelector('#data')?.textContent ?? '{}',
      ) as Data
    } else {
      let dataResponse = await fetch('/data.json')
      dataValue = (await dataResponse.json()) as Data
    }
    set(traverseAndDecode(dataValue))
    loading.set(false)
  }

  // eslint-disable-next-line @typescript-eslint/no-floating-promises
  fetchData()
})
