import path from 'node:path'
import fs from 'node:fs'
import os from 'node:os'

import { platforms } from './platforms.js'

let platform = os.platform()
let arch = os.arch()

let userPlatform = platforms.find(p => p.name === platform)

if (!userPlatform?.arch.includes(arch)) {
  console.error(`Unsupported platform or architecture: ${platform}, ${arch}`)
  process.exit(1)
}

let binaryName = ['todoctor', userPlatform.extension].filter(Boolean).join('.')

let sourcePath = path.join(
  import.meta.dirname,
  '..',
  'target',
  'release',
  binaryName,
)
let destDir = path.join(
  import.meta.dirname,
  '../packages/',
  `${platform}-${arch}`,
)
let destPath = path.join(destDir, binaryName)

if (!fs.existsSync(destDir)) {
  fs.mkdirSync(destDir, { recursive: true })
}

try {
  fs.copyFileSync(sourcePath, destPath)
  fs.chmodSync(destPath, 0o755)
} catch (error) {
  console.error(`Error copying file: ${error.message}`)
  process.exit(1)
}
