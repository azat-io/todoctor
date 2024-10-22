import path from 'node:path'
import fs from 'node:fs'

import { dirname as binDirectory, binaries } from '../bin/todoctor'

Object.values(binaries).forEach(binaryPath => {
  fs.chmodSync(path.join(binDirectory, binaryPath), 0o755)
})
