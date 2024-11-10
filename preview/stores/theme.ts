import { writable } from 'svelte/store'

let userTheme =
  localStorage.getItem('theme') ??
  (globalThis.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light')

export let theme = writable(userTheme)

export let toggleTheme = (): void => {
  theme.update(value => (value === 'dark' ? 'light' : 'dark'))
}

theme.subscribe(value => {
  localStorage.setItem('theme', value)
  globalThis.document.documentElement.dataset.theme = value
})
