# Todoctor

<picture>
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-dark.webp"
    media="(prefers-color-scheme: light)"
  />
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-light.webp"
    media="(prefers-color-scheme: dark)"
  />
  <img
    src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-dark.webp"
    alt="Todoctor Logo"
    align="right"
    height="160"
    width="160"
  />
</picture>

[![Version](https://img.shields.io/npm/v/todoctor.svg?color=2c7f50&labelColor=353c3c)](https://npmjs.com/package/todoctor)
[![GitHub License](https://img.shields.io/badge/license-MIT-232428.svg?color=2c7f50&labelColor=353c3c)](https://github.com/azat-io/todoctor/blob/main/license)

Todoctor is a powerful tool for analyzing, tracking, and visualizing technical debt in your codebase using Git. It collects and monitors `TODO`/`FIXME` comments in your code, allowing you to observe changes over time.

## Features

- Automatically extracts `TODO`, `FIXME`, and other custom tags from your codebase.
- Supports JavaScript and TypeScript programming languages that Git tracks.
- Analyzes each commit to gather and update comment history.
- Integrates with `git blame` to track the authorship and timing of changes.
- Visualizes the history of changes to analyze the growth or reduction of technical debt.

## Usage

Run the tool in the root directory of your project:

```sh
npx todoctor
```

The program will automatically collect data and display the history of `TODO` / `FIXME` comments across commits.

## License

MIT &copy; [Azat S.](https://azat.io)
