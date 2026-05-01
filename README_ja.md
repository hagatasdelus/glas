# 🍁 glas

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg)](https://crates.io/)
[![install size](https://img.shields.io/badge/install_size-0.0_kB-green.svg)](https://packagephobia.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Coverage Status](https://coveralls.io/repos/github/hagatasdelus/glas/badge.svg?branch=main)](https://coveralls.io/github/hagatasdelus/glas?branch=main)

<div align="center" style="font-size: 1.1rem;"><sub>
README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

<br>

glas - Git対応でパイプラインフレンドリーな `ls` 代替ツール

このリポジトリは現在開発中🚧であり、予告なくコマンド仕様や挙動が変更される可能性があります。

## 概要

`glas` は、ファイルシステムの物理的な配置とGitのトラッキング状態をシームレスに統合して表示する次世代の `ls` コマンドです。
変更状態に基づく論理的なグルーピングと、ネストした変更を仮想的にフラット化することで、`ls` と `git status` を行き来する際の認知的負荷を取り除きます。
また、UNIX哲学に基づき設計されており、人間向けには豊富な ANSI カラー表示によるインターフェースを提供しつつ、他のコマンドへパイプしたときには自動的にすべての装飾を取り除き、生のパスだけを出力します。

## インストール

```bash
undefined
```

## 使い方

```bash
git glas [OPTIONS] [FILE]...
    or
glas [OPTIONS] [FILE]...

META OPTIONS:
  -?,                        ヘルプメッセージを表示する
  -v, --version              バージョン情報を表示する

DISPLAY OPTIONS:
  -1, --oneline              1行に1エントリずつ表示する
  -l, --long                 ファイルの拡張メタデータを表形式で表示する
  -h, --header               ロングフォーマット時にヘッダー行を表示する
      --color=WHEN           カラー出力の制御 [always, auto, never] (デフォルト: auto)
      --absolute             エントリを絶対パスで表示する
  -z, --null                 安全なパイプライン処理のため、各行の末尾にNULL文字（\0）を付加する
      --format=FORMAT        カスタム出力フォーマットを指定する（例: '%(size) %(path)'）

FILTERING & SORTING OPTIONS:
  -a, --all                  隠しファイル（.で始まるファイル）を表示する
  -d, --treat-dirs-as-files  ディレクトリをファイルとして一覧表示する（再帰的に中身を表示しない）
  -D, --only-dirs            ディレクトリのみを表示する
  -f, --only-files           ファイルのみを表示する
  -s, --sort FIELD           指定したフィールドでソートする [name, size, time, git] (デフォルト: git, then name)
    
GIT-AWARE OPTIONS:
      --git-only             変更がある追跡対象ファイルのみを表示する 
      --show-ignored         無視されたファイルを表示する
      --no-git               Gitコンテキストの取得を無効にする
      --flatten DEPTH        ネストされた変更ファイルを指定の階層（DEPTH）まで展開して表示する（デフォルト: 0）
      --full-name            プロジェクトのルートディレクトリからの相対パスで出力する
```

## ライセンス

MIT License

## 作者

Horinaka Yoshiki
