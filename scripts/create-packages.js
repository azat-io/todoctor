/* eslint-disable perfectionist/sort-objects */

import fs from 'node:fs/promises'
import path from 'node:path'

import { getPackageJson } from './get-package-json.js'
import { platforms } from './platforms.js'

let packagesDir = path.join(import.meta.dirname, '../packages')
let rootPackageJson = await getPackageJson()

for (let platform of platforms) {
  for (let arch of platform.arch) {
    let packageDir = path.join(packagesDir, `${platform.name}-${arch}`)

    await fs.mkdir(packageDir, {
      recursive: true,
    })

    let name = `@todoctor/${platform.name}-${arch}`

    fs.writeFile(
      path.join(packageDir, 'package.json'),
      JSON.stringify(
        {
          name,
          description: `Platform-specific binary for Todoctor (${platform.name}, ${arch})`,
          version: rootPackageJson.version,
          repository: rootPackageJson.repository,
          author: rootPackageJson.author,
          license: rootPackageJson.license,
          keywords: rootPackageJson.keywords,
          os: [platform.name],
          cpu: [arch],
        },
        null,
        2,
      ),
    )

    fs.writeFile(
      path.join(packageDir, 'readme.md'),
      [
        `# Todoctor (${platform.name[0].toUpperCase() + platform.name.slice(1)}, ${arch.toUpperCase()})`,
        '',
        '<img',
        '  src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-light.webp"',
        '  alt="Todoctor Logo"',
        '  align="right"',
        '  height="160"',
        '  width="160"',
        '/>',
        '',
        `[![Version](https://img.shields.io/npm/v/${name}.svg?color=2c7f50&labelColor=353c3c)](https://npmjs.com/package/${name})`,
        '[![GitHub License](https://img.shields.io/badge/license-MIT-232428.svg?color=2c7f50&labelColor=353c3c)](https://github.com/azat-io/todoctor/blob/main/license)',
        '',
        'This is a platform-specific binary package for Todoctor. It contains the pre-compiled binary executable for your operating system and architecture.',
        '',
        '**Note:** This package is not meant to be installed directly by users. Instead, please install the main Todoctor package, which will automatically download the correct binary for your platform.',
        '',
        '## Usage',
        '',
        '```sh',
        'npx todoctor',
        '```',
        '',
        '## License',
        '',
        'MIT &copy; [Azat S.](https://azat.io)',
      ].join('\n'),
    )
  }
}
