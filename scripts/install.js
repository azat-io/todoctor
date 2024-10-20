import { fileURLToPath } from 'node:url'
import path from 'node:path'
import os from 'node:os'
import fs from 'node:fs'

const filename = fileURLToPath(import.meta.url)
const dirname = path.dirname(filename)

const platform = os.platform()
const arch = os.arch()

const binaries = {
  'win32:x64': 'windows/x64/todoctor.exe',
  'darwin:arm64': 'macos/arm64/todoctor',
  'linux:arm64': 'linux/arm64/todoctor',
  'darwin:x64': 'macos/x64/todoctor',
  'linux:x64': 'linux/x64/todoctor',
}

const key = `${platform}:${arch}`
const relativePath = binaries[key]

if (!relativePath) {
  console.error(`Unsupported platform or architecture: ${platform}, ${arch}`)
  process.exit(1)
}

const binaryPath = path.join(dirname, '../build', relativePath)
const destinationDir = path.join(dirname, '../bin')
const destinationPath = path.join(destinationDir, 'todoctor')

if (!fs.existsSync(destinationDir)) {
  fs.mkdirSync(destinationDir, { recursive: true })
}

try {
  fs.copyFileSync(binaryPath, destinationPath)

  if (platform !== 'win32') {
    fs.chmodSync(destinationPath, '755')
  }

  // eslint-disable-next-line no-console
  console.log(`Installed todoctor binary for ${platform}`)
} catch (error) {
  console.error(`Error during installation: ${error.message}`)
  process.exit(1)
}
