import eslintConfig from '@azat-io/eslint-config-typescript'
import typescriptParser from '@typescript-eslint/parser'
import eslintPluginSvelte from 'eslint-plugin-svelte'
import svelteParser from 'svelte-eslint-parser'

export default [
  ...eslintPluginSvelte.configs['flat/recommended'],
  ...eslintConfig,
  {
    ignores: ['.netlify/**'],
  },
  {
    languageOptions: {
      parserOptions: {
        parser: {
          typescript: typescriptParser,
          ts: typescriptParser,
        },
      },
      parser: svelteParser,
    },

    rules: {
      'svelte/valid-compile': 'off',
    },

    files: ['**/*.svelte'],
  },
]
