import { defineMock } from 'vite-plugin-mock-dev-server'

import { data } from './data'

export default defineMock({
  headers: {
    'Content-Type': 'application/json',
  },
  url: '/data.json',
  body: data,
})
