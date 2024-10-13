import { hydrate, unmount, mount } from 'svelte'

import App from './app.svelte'

let target = document.getElementById('root')!
let render = target.firstChild?.nextSibling ? hydrate : mount

let app = render(App, { target })

// @ts-ignore
window.unmount = () => unmount(app)
