import { hydrate, unmount, mount } from 'svelte'

import App from './app.svelte'

let target = document.querySelector('#root')!
let render = target.firstChild?.nextSibling ? hydrate : mount

let app = render(App, { target })

// @ts-ignore
globalThis.unmount = () => unmount(app)
