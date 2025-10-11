import { readable, writable } from 'svelte/store'

import type { Data } from '~/typings/index.d'

function traverseAndDecode<T>(object: T): T {
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

function decodeHtmlEntities(string: string): string {
  let textArea = document.createElement('textarea')
  textArea.innerHTML = string
  return textArea.value
}

export let loading = writable(true)

export let data = readable<Partial<Data>>({}, set => {
  async function fetchData(): Promise<void> {
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

  // eslint-disable-next-line typescript/no-floating-promises
  fetchData()
})
