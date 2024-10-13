import { readable } from 'svelte/store'

import type { Data } from '~/typings/index.d'

export let data = readable<Partial<Data>>({}, set => {
  let fetchData = async () => {
    if (import.meta.env.MODE === 'production') {
      // @ts-ignore
      set(window.data)
    } else {
      let dataResponse = await fetch('/data.json')
      let dataJson = await dataResponse.json()
      set(dataJson)
    }
  }

  fetchData()
})
