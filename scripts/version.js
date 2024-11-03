import fs from 'node:fs/promises'
import path from 'node:path'

import { getPackageJson } from './get-package-json.js'

let rootPackageJson = await getPackageJson()

let { version } = rootPackageJson

rootPackageJson.optionalDependencies = Object.fromEntries(
  Object.keys(rootPackageJson.optionalDependencies).map(key => [
    key,
    `^${version}`,
  ]),
)

await fs.writeFile(
  path.join(import.meta.dirname, '../package.json'),
  JSON.stringify(rootPackageJson, null, 2),
)
