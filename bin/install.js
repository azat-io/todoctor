import path from 'node:path'
import os from 'node:os'
import fs from 'node:fs'

let platform = os.platform()
let arch = os.arch()
let binaryPath

if (platform === 'linux' && arch === 'x64') {
  binaryPath = path.join(import.meta.dirname, 'linux/x64/todoctor')
} else if (platform === 'linux' && arch === 'arm64') {
  binaryPath = path.join(import.meta.dirname, 'linux/arm64/todoctor')
} else if (platform === 'darwin' && arch === 'x64') {
  binaryPath = path.join(import.meta.dirname, 'macos/x64/todoctor')
} else if (platform === 'darwin' && arch === 'arm64') {
  binaryPath = path.join(import.meta.dirname, 'macos/arm64/todoctor')
} else if (platform === 'win32' && arch === 'x64') {
  binaryPath = path.join(import.meta.dirname, 'windows/x64/todoctor.exe')
} else if (platform === 'win32' && arch === 'arm64') {
  binaryPath = path.join(import.meta.dirname, 'windows/arm64/todoctor.exe')
} else {
  console.error(`Unsupported platform or architecture: ${platform}, ${arch}`)
  process.exit(1)
}

const destinationPath = path.join(import.meta.dirname, 'todoctor')

fs.copyFileSync(binaryPath, destinationPath)

if (platform !== 'win32') {
  fs.chmodSync(destinationPath, '755')
}

// eslint-disable-next-line no-console
console.log(`Installed todoctor binary for ${platform}`)
