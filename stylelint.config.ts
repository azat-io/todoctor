import type { Config } from 'stylelint'

export default {
  overrides: [
    {
      customSyntax: 'postcss-html',
      files: ['**/*.svelte'],
    },
  ],
  extends: '@azat-io/stylelint-config',
  ignoreFiles: ['dist/**/*'],
} satisfies Config
