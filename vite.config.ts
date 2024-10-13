import type { Plugin } from 'vite'

import { viteSingleFile as singleFile } from 'vite-plugin-singlefile'
import mockDevServer from 'vite-plugin-mock-dev-server'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { browserslistToTargets } from 'lightningcss'
import { minify } from 'html-minifier-terser'
import browserslist from 'browserslist'
import { defineConfig } from 'vite'
import path from 'node:path'
import os from 'node:os'

let htmlMinify = (): Plugin => ({
  transformIndexHtml: async html => {
    try {
      return await minify(html, {
        removeStyleLinkTypeAttributes: true,
        removeScriptTypeAttributes: true,
        removeRedundantAttributes: true,
        collapseWhitespace: true,
        useShortDoctype: true,
        removeComments: true,
        minifyCSS: true,
        minifyJS: true,
      })
    } catch (_error) {
      return html
    }
  },
  name: 'vite-plugin-html-minify',
})

export default defineConfig({
  plugins: [
    mockDevServer({
      include: ['preview/mock/**/*.mock.ts'],
      prefix: ['^/data.json$'],
    }),
    singleFile({
      inlinePattern: ['**/*.js', '**/*.css'],
      useRecommendedBuildConfig: true,
      removeViteModuleLoader: true,
      deleteInlinedFiles: true,
    }),
    htmlMinify(),
    svelte(),
  ],
  css: {
    lightningcss: {
      targets: browserslistToTargets(
        browserslist(null, {
          config: path.join(import.meta.dirname, '.browserslistrc'),
        }),
      ),
    },
    transformer: 'lightningcss',
  },
  resolve: {
    alias: {
      '~': path.join(import.meta.dirname, 'preview'),
    },
    extensions: ['.js', '.ts', '.svelte', '.css'],
  },
  build: {
    rollupOptions: {
      external: ['data.json'],
    },
    outDir: path.join(import.meta.dirname, 'dist'),
  },
  server: {
    host: os.networkInterfaces().eth0?.[0].address,
    port: 3000,
  },
  root: path.join(import.meta.dirname, 'preview'),
  esbuild: {
    legalComments: 'none',
  },
  base: './',
})
