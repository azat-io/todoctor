import { writable } from 'svelte/store'

let userTheme =
  localStorage.getItem('theme') ||
  (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')

export let theme = writable(userTheme)

export let toggleTheme = () => {
  theme.update(value => (value === 'dark' ? 'light' : 'dark'))
}

theme.subscribe(value => {
  localStorage.setItem('theme', value)
  window.document.documentElement.setAttribute('data-theme', value)
})
