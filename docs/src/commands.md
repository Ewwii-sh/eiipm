# Commands

Eiipm offers many commands which users can use to manage ewwii and its packages.

## Overview

Here is a simple overview before we get started.

| Command               | Aliases | Flags / Options                    |
| --------------------- | ------- | ---------------------------------- |
| `install <PACKAGE>`   | `i`     | `--debug`                          |
| `uninstall <PACKAGE>` | `rm`    | `--debug`                          |
| `update [PACKAGE]`    | `up`    | `--debug`                          |
| `list`                | `l`     | `-v`, `-t`, `-q <NAME>`, `--debug` |
| `help`                | None    | None                               |
| `-V, --version`       | None    | None                               |

**Flags for `list`:**

- `-v, --verbose`: verbose output
- `-t, --total-count`: output just total package count
- `-q, --query <NAME>`: query a package (works with `--verbose`)
- `--debug`: debug logs
