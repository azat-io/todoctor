# Todoctor

<picture>
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-light.webp"
    media="(prefers-color-scheme: light)"
  />
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-dark.webp"
    media="(prefers-color-scheme: dark)"
  />
  <img
    src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/logo-light.webp"
    alt="Todoctor Logo"
    align="right"
    height="160"
    width="160"
  />
</picture>

[![Version](https://img.shields.io/npm/v/todoctor.svg?color=2c7f50&labelColor=353c3c)](https://npmjs.com/package/todoctor)
[![GitHub License](https://img.shields.io/badge/license-MIT-232428.svg?color=2c7f50&labelColor=353c3c)](https://github.com/azat-io/todoctor/blob/main/license)

Todoctor is a powerful tool for analyzing, tracking, and visualizing technical
debt in your codebase using Git.

It collects and monitors `TODO`/`FIXME` comments in your code, allowing you to
observe changes over time.

## Why

Developers often leave `TODO` comments in the code to highlight areas that need
improvement or refactoring. However, these comments are rarely converted into
tasks in tracking systems.

As a result, todos remain hidden in the codebase and can sit there for years
without attention, leading to a hidden backlog of work. This tool tracks these
todo comments and prevents them from being forgotten.

## Features

- Automatically extracts `TODO`, `FIXME`, and other tags from your codebase.
- Supports JavaScript and TypeScript programming languages that Git tracks.
- Analyzes each commit to gather and update comment history.
- Integrates with `git blame` to track the authorship and timing of changes.
- Visualizes the history of changes to analyze the growth or reduction of
  technical debt.

## Usage

Run the tool in the root directory of your project:

```sh
npx todoctor
```

The program will automatically collect data and display the history of `TODO` /
`FIXME` comments across commits.

## Report

See an [example report](https://todoctor.azat.io).

### Todos Graph

After running the tool, it generates a detailed graph showing the evolution of
TODO comments over time. The graph visualizes how many todo comments were added,
resolved, or modified across the project's history.

This helps you track the technical debt and maintenance progress at a glance.

<picture>
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/graph-light.webp"
    media="(prefers-color-scheme: light)"
  />
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/graph-dark.webp"
    media="(prefers-color-scheme: dark)"
  />
  <img
    src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/graph-light.webp"
    alt="Todoctor Graph Example"
    style="margin: 24px 0"
  />
</picture>

### Additional Information

In addition to the graph, the tool provides insightful statistics, such as:

- The total number of todo comments.
- The age of the oldest todo.
- The average age of all todos.
- The author with the highest number of todo comments.

These insights help you better understand the state of your codebase and
prioritize refactoring efforts.

<picture>
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/info-light.webp"
    media="(prefers-color-scheme: light)"
  />
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/info-dark.webp"
    media="(prefers-color-scheme: dark)"
  />
  <img
    src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/info-light.webp"
    alt="Todoctor Info Example"
    style="margin: 24px 0"
  />
</picture>

### List of Todos

Finally, the tool generates a detailed list of all todo comments in your project
in a tabular format.

The list includes the comment text, the file path, and additional metadata, such
as line numbers and authorship information. This list helps you identify,
review, and manage unresolved tasks more effectively.

<picture>
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/list-light.webp"
    media="(prefers-color-scheme: light)"
  />
  <source
    srcset="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/list-dark.webp"
    media="(prefers-color-scheme: dark)"
  />
  <img
    src="https://raw.githubusercontent.com/azat-io/todoctor/main/assets/list-light.webp"
    alt="Todoctor List Example"
    style="margin: 24px 0"
  />
</picture>

## Options

Todoctor supports the following command-line options:

### --months

Specifies the number of months to include when tracking TODOs in the repository.
If not provided, defaults to 3 months.

Example:

```sh
todoctor --months 6
```

### --ignore

Allows you to specify files or directories to ignore during the analysis. The
files in your `.gitignore` are ignored by default, you don't need to ignore them
additionally. This option can be used multiple times.

Example:

```sh
todoctor --ignore src/deprecated/ --ignore tests/legacy.test.js
```

### --include-keywords

Allows you to specify additional keywords in comments that will be treated as
technical debt. This option can be used multiple times.

Example:

```sh
todoctor --include-keywords eslint-disable-next-line
```

### --exclude-keywords

Allows you to exclude keywords from the report. By default, the following
keywords are used to define the technical debt comment:

- `TODO`
- `FIXME`
- `XXX`
- `HACK`
- `BUG`
- `OPTIMIZE`
- `REFACTOR`
- `TEMP`
- `CHANGED`
- `IDEA`
- `NOTE`
- `REVIEW`
- `NB`
- `QUESTION`
- `DEBUG`
- `KLUDGE`
- `COMPAT`
- `WARNING`
- `DANGER`
- `INFO`
- `DEPRECATED`
- `COMBAK`

Example:

```sh
todoctor --exclude-keywords WARNING --exclude-keywords DEPRECATED
```

### --output-format

You can specify the format of the report. Possible options are `html`, `json`
and `csv`. The default value is `html`.

Example:

```sh
todoctor --output-format json
```

### --output

You can define the folder where the report file will be saved. By default it is
`todoctor` folder in the project root.

Example:

```sh
todoctor --output report
```

### --help

Displays this help message with available options.

### --version

Displays the current version of Todoctor.

## License

MIT &copy; [Azat S.](https://azat.io)
