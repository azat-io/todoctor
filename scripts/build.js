import path from 'node:path'
import fs from 'node:fs'
import os from 'node:os'

let platform = os.platform()
let arch = os.arch()

let platformMap = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}

let archMap = {
  arm64: 'arm64',
  x64: 'x64',
}

let platformName = platformMap[platform]
let archName = archMap[arch]

if (!platformName || !archName) {
  console.error(`Unsupported platform or architecture: ${platform}, ${arch}`)
  process.exit(1)
}

let binaryName = platform === 'win32' ? 'todoctor.exe' : 'todoctor'

let sourcePath = path.join(
  import.meta.dirname,
  '..',
  'target',
  'release',
  binaryName,
)
let destDir = path.join(
  import.meta.dirname,
  '..',
  'bin',
  platformName,
  archName,
)
let destPath = path.join(destDir, binaryName)

if (!fs.existsSync(destDir)) {
  fs.mkdirSync(destDir, { recursive: true })
}

try {
  fs.copyFileSync(sourcePath, destPath)

  if (platform !== 'win32') {
    fs.chmodSync(destPath, 0o755)
  }
} catch (error) {
  console.error(`Error copying file: ${error.message}`)
  process.exit(1)
}
