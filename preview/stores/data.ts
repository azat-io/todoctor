import { readable, writable } from 'svelte/store'

import type { Data } from '~/typings/index.d'

export let loading = writable(true)

export let data = readable<Partial<Data>>({}, set => {
  let fetchData = async () => {
    if (import.meta.env.MODE === 'production') {
      // @ts-ignore
      set(window.data)
      loading.set(false)
    } else {
      let dataResponse = await fetch('/data.json')
      let dataJson = await dataResponse.json()
      set(dataJson)
      loading.set(false)
    }
  }

  fetchData()
})
