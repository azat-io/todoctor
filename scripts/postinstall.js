import { fileURLToPath } from 'node:url'
import path from 'node:path'
import fs from 'node:fs'

import { binaries } from '../bin/todoctor'

let filename = fileURLToPath(import.meta.url)
let dirname = path.dirname(filename)

Object.values(binaries).forEach(binaryPath => {
  fs.chmodSync(path.join(dirname, '../bin/', binaryPath), 0o755)
})
