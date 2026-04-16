# 🍁 glas

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg)](https://crates.io/)
[![install size](https://img.shields.io/badge/install_size-0.0_kB-green.svg)](https://packagephobia.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<div align="center" style="font-size: 1.1rem; margin-bottom: 1rem;"><sub>
Package README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

glas - A Git-aware, pipeline-friendly `ls` alternative for modern developers.

## Description

`glas` is a next-generation terminal file lister that seamlessly merges the physical file system with your Git working tree state.
By logically grouping files by their Git status and virtually flattening nested modifications, it eliminates the cognitive overhead of switching between `ls` and `git status`.
Built strictly upon the UNIX philosophy, it provides a rich, ANSI-colorized interface for humans while automatically stripping all formatting to yield raw paths when piped to other commands.

## Usage

```bash
# Default behavior (Grid layout)
# Displays files in a standard grid, utilizing ANSI colors to indicate Git status 
# (e.g., Modified, Untracked).
glas

# Detailed list view (-l)
# Shows a single-column list with a dedicated Git status column, size, date, and path.
glas -l

# --- Example Output (Illustrative only) ---
# GIT   PERMS       SIZE    DATE         NAME
# [M ]  -rw-r--r--   12 KB  2 hours ago  src/deep/nested/core.rs
# [??]  -rw-r--r--     0 B  Just now     new_file.txt
# [  ]  drwxr-xr-x       -  May 12       src/
# ------------------------------------------

# Pipeline friendly (TTY detection)
# When piped, all visual metadata is purged automatically. 
# It outputs only raw, newline-separated file paths.
glas | fzf | xargs vim

# Null-separated output for paths with spaces
glas -z | xargs -0 rm
```

## Lisence

MIT Lisence

## Author

Horinaka Yoshiki
