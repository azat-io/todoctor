import type { Linter } from 'eslint'

import eslintConfig from '@azat-io/eslint-config'

export default eslintConfig({
  perfectionist: true,
  typescript: true,
  svelte: true,
  node: true,
}) satisfies Promise<Linter.Config[]>
