# 🍁 glas

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg)](https://crates.io/)
[![install size](https://img.shields.io/badge/install_size-0.0_kB-green.svg)](https://packagephobia.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Coverage Status](https://coveralls.io/repos/github/hagatasdelus/glas/badge.svg?branch=main)](https://coveralls.io/github/hagatasdelus/glas?branch=main)
[![Version](https://img.shields.io/badge/Version-0.1.2-blue.svg)](https://github.com/hagatasdelus/glas/releases/tag/v0.1.2)

<div align="center" style="font-size: 1.1rem;"><sub>
README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

<br>

glas - A Git-aware, pipeline-friendly `ls` alternative.

This repository is currently a WIP 🚧, and command behavior and interfaces may change without notice.

## Overview

`glas` is a next-generation terminal file lister that seamlessly merges the physical file system with your Git working tree state.
By logically grouping files by their Git status and virtually flattening nested modifications, it eliminates the cognitive overhead of switching between `ls` and `git status`.
Built strictly upon the UNIX philosophy, it provides a rich, ANSI-colorized interface for humans while automatically stripping all formatting to yield raw paths when piped to other commands.

## Installation

```bash
brew install hagatasdelus/tap/glas
```

## Usage

```bash
glas [OPTIONS] [FILE]...

Options:
  -h, --help             Print help
  -V, --version          Print version
      --completions      generate completion files

DISPLAY OPTIONS:
  -1, --oneline          display one entry per line
  -l, --long             display extended file metadata as a table
  -H, --header           display a header row in long format
      --color <COLOR>    [default: auto] [possible values: always, auto, never]
      --absolute         display entries with their absolute paths
  -z, --null             terminate lines with a null byte
      --format <FORMAT>  custom output format

FILTERING AND SORTING OPTIONS:
  -a, --all                  show hidden and dot-files
  -d, --treat-dirs-as-files  list directories as files
  -D, --only-dirs            list only directories
  -f, --only-files           list only files
  -s, --sort <SORT>          [possible values: name, size, time, git]

GIT-AWARE OPTIONS:
      --cached               show cached files in index (default)
      --stage                show staged contents' mode bits, object name and stage number
      --deleted              show files with an unstaged deletion
      --modified             show files with an unstaged modification
      --others               show other (untracked) files in the output
      --ignored              show only ignored files in the output
      --include-ignored      include ignored files in output
      --no-git               disable Git context fetching
      --flatten[=<FLATTEN>]  flatten nested modified files [default: 0]
      --full-name            output paths relative to repo root
```

## Lisence

MIT Lisence

## Author

Horinaka Yoshiki
