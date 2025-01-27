#!/usr/bin/env node

import { createRequire } from 'node:module'
import { spawn } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import os from 'node:os'

let require = createRequire(import.meta.url)
let filename = fileURLToPath(import.meta.url)
let dirname = path.dirname(filename)

let platform = os.platform()
let arch = os.arch()

let packageName = `@todoctor/${platform}-${arch}`
let binaryPath

try {
  binaryPath = require.resolve(packageName)
} catch (error) {
  console.error(`Failed to find binary for ${packageName}`)
  console.error(error)
  process.exit(1)
}

let distributionPath = path.join(dirname, '../dist')
let arguments_ = process.argv.slice(2)
let environment = { ...process.env, DIST_PATH: distributionPath }
let child = spawn(binaryPath, arguments_, {
  stdio: 'inherit',
  env: environment,
})

child.on('exit', code => {
  process.exit(code)
})
