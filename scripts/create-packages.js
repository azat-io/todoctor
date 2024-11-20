import fs from 'node:fs/promises'
import path from 'node:path'

import { getPackageJson } from './get-package-json.js'
import { platforms } from './platforms.js'

let packagesDirectory = path.join(import.meta.dirname, '../packages')
let rootPackageJson = await getPackageJson()

let packageTypes = []

for (let platform of platforms) {
  for (let arch of platform.arch) {
    packageTypes.push([platform.name, arch])
  }
}

await Promise.all(
  packageTypes.map(async ([platform, arch]) => {
    let packageDirectory = path.join(packagesDirectory, `${platform}-${arch}`)

    await fs.mkdir(packageDirectory, {
      recursive: true,
    })

    let name = `@todoctor/${platform}-${arch}`
    let displayName = platform.toUpperCase() + platform.slice(1)
    let displayArch = arch.toUpperCase()

    fs.writeFile(
      path.join(packageDirectory, 'package.json'),
      /* eslint-disable perfectionist/sort-objects */
      JSON.stringify(
        {
          name,
          version: rootPackageJson.version,
          description: `Platform-specific binary for Todoctor (${displayName}, ${displayArch})`,
          keywords: rootPackageJson.keywords,
          repository: rootPackageJson.repository,
          license: rootPackageJson.license,
          author: rootPackageJson.author,
          os: [platform.name],
          cpu: [arch],
        },
        null,
        2,
      ),
      /* eslint-enable perfectionist/sort-objects */
    )

    fs.writeFile(
      path.join(packageDirectory, 'readme.md'),
      [
        `# Todoctor (${displayName}, ${displayArch})`,
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
  }),
)
