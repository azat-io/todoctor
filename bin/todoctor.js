#!/usr/bin/env node

import { spawn } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import os from 'node:os'
import fs from 'node:fs'

let filename = fileURLToPath(import.meta.url)
let dirname = path.dirname(filename)

let platform = os.platform()
let arch = os.arch()

let binaries = {
  'win32:x64': 'windows/x64/todoctor.exe',
  'darwin:arm64': 'macos/arm64/todoctor',
  'linux:arm64': 'linux/arm64/todoctor',
  'darwin:x64': 'macos/x64/todoctor',
  'linux:x64': 'linux/x64/todoctor',
}

let key = `${platform}:${arch}`
let relativePath = binaries[key]

if (!relativePath) {
  console.error(`Unsupported platform or architecture: ${platform}, ${arch}`)
  process.exit(1)
}

let binaryPath = path.join(dirname, relativePath)

if (!fs.existsSync(binaryPath)) {
  console.error(`Binary not found: ${binaryPath}`)
  console.error(
    'Please ensure that the postinstall script has run successfully.',
  )
  process.exit(1)
}

let args = process.argv.slice(2)
let child = spawn(binaryPath, args, { stdio: 'inherit' })

child.on('exit', code => {
  process.exit(code)
})
