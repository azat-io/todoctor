import fs from 'node:fs/promises'
import path from 'node:path'

export async function getPackageJson() {
  let rootPackageJsonPath = path.join(import.meta.dirname, '../package.json')
  return JSON.parse(await fs.readFile(rootPackageJsonPath, 'utf8'))
}
