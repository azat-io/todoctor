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
  let packageBinaryFile = 'todoctor'
  if (platform === 'win32') {
    packageBinaryFile += '.exe'
  }
  binaryPath = require.resolve(`${packageName}/${packageBinaryFile}`)
} catch (error) {
  console.error(`Failed to find binary for ${packageName}`)
  console.error(error)
  process.exit(1)
}

let distPath = path.join(dirname, '../dist')
let args = process.argv.slice(2)
let env = { ...process.env, DIST_PATH: distPath }
let child = spawn(binaryPath, args, { stdio: 'inherit', env })

child.on('exit', code => {
  process.exit(code)
})
