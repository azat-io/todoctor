import { sveltePreprocess } from 'svelte-preprocess'

export default {
  compilerOptions: {
    cssHash: ({ hash, css }) => `s-${hash(css)}`,
    discloseVersion: false,
  },
  preprocess: sveltePreprocess(),
}
