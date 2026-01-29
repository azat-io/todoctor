import type { Plugin } from 'vite'

import { mockDevServerPlugin as mockDevelopmentServer } from 'vite-plugin-mock-dev-server'
import { viteSingleFile as singleFile } from 'vite-plugin-singlefile'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { browserslistToTargets } from 'lightningcss'
import { minify } from 'html-minifier-terser'
import browserslist from 'browserslist'
import { defineConfig } from 'vite'
import fs from 'node:fs/promises'
import path from 'node:path'
import os from 'node:os'

import { data } from './preview/mock/data'

function htmlMinify(): Plugin {
  return {
    transformIndexHtml: async (html): Promise<void> => {
      try {
        await minify(html, {
          removeStyleLinkTypeAttributes: true,
          removeScriptTypeAttributes: true,
          removeRedundantAttributes: true,
          collapseWhitespace: true,
          useShortDoctype: true,
          removeComments: true,
          minifyCSS: true,
          minifyJS: true,
        })
      } catch (error) {
        console.error('HTML minification failed', error)
      }
    },
    name: 'vite-plugin-html-minify',
  }
}

function createFilePlugin(filename: string, content: string): Plugin {
  return {
    generateBundle: async () => {
      let outputDirectory = path.resolve(import.meta.dirname, 'dist')

      try {
        await fs.mkdir(outputDirectory, { recursive: true })
        let filePath = path.join(outputDirectory, filename)

        await fs.writeFile(filePath, content, 'utf8')
      } catch (error) {
        console.error(`Creation file ${filename}:`, error)
      }
    },
    name: 'vite-plugin-create-file',
    apply: 'build',
  }
}

function injectBeforeHead(html: string): Plugin {
  return {
    transformIndexHtml: (htmlValue: string) =>
      htmlValue.replace('</head>', `${html}</head>`),
    name: 'vite-plugin-inject-before-head',
  }
}

let isDocumentation = process.env['DOCS'] === 'true'

export default defineConfig({
  plugins: [
    mockDevelopmentServer({
      include: ['**/*.mock.ts'],
      prefix: ['/data.json'],
      dir: 'preview/mock',
    }),
    ...(isDocumentation ?
      [
        injectBeforeHead(
          `<script id="data" type="application/json">${JSON.stringify(data)}</script>`,
        ),
        injectBeforeHead(
          '<script src="https://cdn.usefathom.com/script.js" data-site="RKLPHYLK" defer></script>',
        ),
        createFilePlugin(
          '_redirects',
          'https://todoctor.netlify.app/* https://todoctor.azat.io/:splat 301!',
        ),
        createFilePlugin(
          '_headers',
          `/*
  content-security-policy: default-src 'self'; script-src 'self' cdn.usefathom.com 'unsafe-inline' 'unsafe-eval' data:; style-src 'self' 'unsafe-inline'; connect-src 'self' cdn.usefathom.com; img-src 'self' data: cdn.usefathom.com;
  strict-transport-security: max-age=63072000; includeSubDomains; preload;`,
        ),
      ]
    : [
        singleFile({
          inlinePattern: ['**/*.js', '**/*.css'],
          useRecommendedBuildConfig: true,
          removeViteModuleLoader: true,
          deleteInlinedFiles: true,
        }),
      ]),
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
    host: os.networkInterfaces()['eth0']?.[0]?.address,
    port: 3000,
  },
  root: path.join(import.meta.dirname, 'preview'),
  esbuild: {
    legalComments: 'none',
  },
  base: './',
})
