import fs from 'node:fs/promises'
import path from 'node:path'

export let getPackageJson = async () => {
  let rootPackageJsonPath = path.join(import.meta.dirname, '../package.json')
  return JSON.parse(await fs.readFile(rootPackageJsonPath, 'utf8'))
}
