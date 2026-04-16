# 🍁 glas

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg)](https://crates.io/)
[![install size](https://img.shields.io/badge/install_size-0.0_kB-green.svg)](https://packagephobia.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<div align="center" style="font-size: 1.1rem; margin-bottom: 1rem;"><sub>
Package README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

glas - 開発者のための、Gitコンテキスト統合・パイプライン完全互換の `ls` 代替ツール。

## Description

`glas` は、ファイルシステムの物理的な配置とGitのトラッキング状態をシームレスに統合して表示する次世代の `ls` コマンドです。
変更状態に基づく論理的なグルーピングと、深い階層の変更をルートに引き上げる「仮想平坦化」により、探索の手間を大幅に削減します。
また、UNIX哲学に基づき設計されており、人間に対しては視認性の高いカラーリングを提供する一方で、パイプ処理（`|`）時には自動的に装飾を排除して純粋なパスのみを出力するため、既存のツールチェインと完全に結合可能です。

## Usage

```bash
# デフォルトの挙動（グリッド表示）
# 従来のlsのようにグリッドで一覧表示し、ファイル名の色付けによってGitステータスを表現します。
glas

# 詳細リスト表示 (-l)
# Gitステータスカラム、権限、サイズ、更新日時を含む詳細なリスト形式で出力します。
glas -l

# --- 出力例（※あくまで仮のイメージ） ---
# GIT   PERMS       SIZE    DATE         NAME
# [M ]  -rw-r--r--   12 KB  2 hours ago  src/deep/nested/core.rs
# [??]  -rw-r--r--     0 B  Just now     new_file.txt
# [  ]  drwxr-xr-x       -  May 12       src/
# --------------------------------------------------------------------------

# パイプライン結合（TTY検出）
# パイプやリダイレクトで渡された場合、すべての装飾やメタデータを自動的にパージし、
# 改行区切りのファイルパスのみを出力します。
glas | fzf | xargs vim

# スペースを含むパスを安全に処理するためのNull文字区切り
glas -z | xargs -0 rm
```
