# 🍁 glas

[![Version](https://img.shields.io/github/v/release/hagatasdelus/glas?label=Version&color=blue)](https://github.com/hagatasdelus/glas/releases)
[![CI](https://github.com/hagatasdelus/glas/actions/workflows/build.yaml/badge.svg)](https://github.com/hagatasdelus/glas/actions/workflows/build.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Git-aware `ls` alternative.

<div style="font-size: 1.2rem;"><sub>
README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

<br>

`glas` is a directory lister that combines filesystem entries with Git status (staged, modified, untracked, ignored). It displays physical files and their working tree status directly in your terminal.

## Usage

``` console
$ glas                     # List entries with Git status coloring
$ glas -l                  # Long format (permissions, size, date, Git status)
$ glas -l -H               # Long format with header row
$ glas --flatten           # Flatten nested modified/untracked files into the view
$ glas --modified          # List only modified files
$ glas --cached            # List only tracked files in the Git index
$ glas --stage             # Show staged contents (mode, object hash, stage number)
$ glas -D                  # List only directories
$ glas -f                  # List only files
$ glas --full-name         # Output paths relative to the repository root
$ glas -z                  # Terminate entries with NUL byte (\0) for xargs -0
```

## Install

**homebrew-tap:**

``` console
$ brew install hagatasdelus/tap/glas
```

**manually:**

Download prebuilt binaries from the [releases page](https://github.com/hagatasdelus/glas/releases).

## Features

### Git Status Integration
`glas` inspects the Git repository and highlights entries according to their working tree status:
- **Modified**: files with unstaged changes (yellow)
- **Staged**: changes ready to commit (green)
- **Untracked**: new files not in Git (blue / cyan)
- **Ignored**: files matched by `.gitignore` (dim / gray)

Key project files such as `Cargo.toml`, `README.md`, and `justfile` are displayed in **bold** for immediate recognition.

Specifying an explicit file target (e.g., `glas Cargo.toml`) also applies Git status styling to that entry.

### Directory Flattening (`--flatten`)
By default, `ls` only lists immediate entries in the current directory. With `--flatten`, `glas` surfaces nested modifications from subdirectories into the current view:

``` console
$ glas --flatten           # Flatten 1 level of nested modifications
$ glas --flatten=2         # Flatten up to 2 levels deep
$ glas --flatten=all       # Recursively flatten all nested modifications
```

Directories with extensive modifications fall back to a summary row (e.g., `src/ (12 modified files)`) to prevent terminal flooding.

### Git Selectors (`--cached`, `--stage`, etc.)
`glas` provides filtering options aligned with `git ls-files`:
- `--cached`: lists only tracked files in the index.
- `--stage`: prints mode bits, Git object SHA, and stage number.
- `--modified`: lists only files with unstaged modifications.
- `--deleted`: lists only files deleted in the working tree.
- `--others`: lists untracked files.
- `--ignored`: lists only ignored files.
- `--include-ignored`: includes ignored files in the listing.

> [!NOTE]
> Passing `--cached` or `--stage` operates on the Git index (file entries) and automatically implies `-f` (`--only-files`), filtering out directories.

### Pipeline-Safe Output
`glas` automatically adjusts its output based on stdout:
- When connected to a terminal (TTY), `glas` outputs colored grid or multi-column formats.
- When piped or redirected, `glas` disables colors and grid layout, falling back to a clean, single-column newline-delimited stream.
- Use `-z` / `--null` to terminate entries with a NUL byte (`\0`), making it safe for `xargs -0` or scripts dealing with paths containing spaces or newlines.
- Use `--format="%(size) %(path)"` to extract specific fields directly.

### Non-Git Directory Fallback
When executed outside a Git repository, `glas` gracefully falls back to a standard directory lister (grid layout, file metadata, permissions, sorting) without issuing errors or warnings.

## Shell Completion

Generate shell completion scripts with `--completions`:

``` console
$ glas --completions
```

Supported shells: `bash`, `elvish`, `fish`, `powershell`, `zsh`.

## Options

### DISPLAY OPTIONS
| Flag | Description |
| :--- | :--- |
| `-1`, `--oneline` | Display one entry per line |
| `-l`, `--long` | Display extended file metadata as a table |
| `-H`, `--header` | Display a header row in long format |
| `--color <COLOR>` | Control color output (`always`, `auto`, `never`) [default: `auto`] |
| `--absolute` | Display entries with their absolute paths |
| `-z`, `--null` | Terminate lines with a null byte (`\0`) |
| `--format <FORMAT>` | Custom output format (e.g., `%(size) %(path)`) |

### FILTERING AND SORTING OPTIONS
| Flag | Description |
| :--- | :--- |
| `-a`, `--all` | Show hidden and dot-files |
| `-d`, `--treat-dirs-as-files` | List directories as files (do not recurse) |
| `-D`, `--only-dirs` | List only directories |
| `-f`, `--only-files` | List only files |
| `-s`, `--sort <SORT>` | Sort by field (`name`, `size`, `time`, `git`) [default: `git`] |

### GIT-AWARE OPTIONS
| Flag | Description |
| :--- | :--- |
| `--cached` | Show cached files in index (implies `-f`) |
| `--stage` | Show staged mode bits, object hash, and stage number |
| `--deleted` | Show files with an unstaged deletion |
| `--modified` | Show files with an unstaged modification |
| `--others` | Show other (untracked) files in the output |
| `--ignored` | Show only ignored files in the output |
| `--include-ignored` | Include ignored files in output |
| `--no-git` | Disable Git context fetching |
| `--flatten[=<DEPTH>]` | Flatten nested modified files (`0`, `1`, ..., `all`) [default: `0`] |
| `--full-name` | Output paths relative to repo root |

## License

[MIT](LICENSE)

## Author

[Hagata](https://github.com/hagatasdelus)
