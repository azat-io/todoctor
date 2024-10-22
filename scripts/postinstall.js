import { fileURLToPath } from 'node:url'
import path from 'node:path'
import fs from 'node:fs'

import { binaries } from '../bin/todoctor.js'

let filename = fileURLToPath(import.meta.url)
let dirname = path.dirname(filename)

Object.values(binaries).forEach(binaryPath => {
  let fullBinaryPath = path.join(dirname, '../bin/', binaryPath)
  if (fs.existsSync(fullBinaryPath)) {
    fs.chmodSync(path.join(dirname, '../bin/', binaryPath), 0o755)
  }
})
