import fs from 'node:fs/promises'
import path from 'node:path'
import os from 'node:os'

import { platforms } from './platforms.js'

let platform = os.platform()
let arch = os.arch()

let userPlatform = platforms.find(
  currentPlatform => currentPlatform.name === platform,
)

if (!userPlatform?.arch.includes(arch)) {
  throw new Error(`Unsupported platform or architecture: ${platform}, ${arch}`)
}

let binaryName = ['todoctor', userPlatform.extension].filter(Boolean).join('.')

let sourcePath = path.join(
  import.meta.dirname,
  '..',
  'target',
  'release',
  binaryName,
)
let destinationDirectory = path.join(
  import.meta.dirname,
  '../packages/',
  `${platform}-${arch}`,
)
let destinationPath = path.join(destinationDirectory, binaryName)

if (!(await fs.exists(destinationDirectory))) {
  await fs.mkdir(destinationDirectory, { recursive: true })
}

try {
  await fs.copyFile(sourcePath, destinationPath)
  await fs.chmod(destinationPath, 0o755)
} catch (error) {
  throw new Error(`Error copying file: ${error.message}`)
}
