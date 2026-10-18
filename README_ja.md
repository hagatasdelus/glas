# 🍁 glas

[![Version](https://img.shields.io/github/v/release/hagatasdelus/glas?label=Version&color=blue)](https://github.com/hagatasdelus/glas/releases)
[![CI](https://github.com/hagatasdelus/glas/actions/workflows/build.yaml/badge.svg)](https://github.com/hagatasdelus/glas/actions/workflows/build.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Git対応の `ls` 代替ツール。

<div style="font-size: 1.2rem;"><sub>
README: <a href="./README.md">English</a> | <a href="./README_ja.md">日本語</a>
</sub></div>

<br>

`glas` は、ディレクトリのファイル一覧に Git の作業ツリー状態（ステージ済み、変更あり、未追跡、無視など）を統合して表示するツールです。ファイルシステムの物理的なエントリと Git ステータスをターミナル上で直接確認できます。

## Usage

``` console
$ glas                     # Gitステータスに応じたカラーでファイル一覧を表示
$ glas -l                  # 詳細表示（パーミッション、サイズ、更新日時、Gitステータス）
$ glas -l -H               # 詳細表示（ヘッダー行付き）
$ glas --flatten           # サブディレクトリ内の変更ファイルを現在の階層に展開して表示
$ glas --modified          # 変更のあるファイルのみを表示
$ glas --cached            # Gitインデックスで追跡中のファイルのみを表示
$ glas --stage             # ステージング情報（モード、オブジェクトハッシュ、ステージ番号）を表示
$ glas -D                  # ディレクトリのみを表示
$ glas -f                  # ファイルのみを表示
$ glas --full-name         # リポジトリルートからの相対パスで出力
$ glas -z                  # 出力末尾をNULL文字（\0）にして出力（xargs -0 向け）
```

## Install

**homebrew-tap:**

``` console
$ brew install hagatasdelus/tap/glas
```

**manually:**

[releases page](https://github.com/hagatasdelus/glas/releases) からビルド済みバイナリをダウンロードしてください。

## Features

### Git Status Integration
`glas` は Git リポジトリを検査し、作業ツリーの状態に応じて各エントリを強調表示します：
- **Modified**: 未ステージの変更があるファイル（黄色）
- **Staged**: ステージ済みの変更（緑色）
- **Untracked**: Git 未追跡の新規ファイル（青色 / シアン）
- **Ignored**: `.gitignore` の対象ファイル（減衰色 / 灰色）

`Cargo.toml`、`README.md`、`justfile` などのプロジェクト固有の重要なファイルは、状態に関わらず **太字** で表示されます。

特定のファイルを引数で明示的に指定した場合（例: `glas Cargo.toml`）も、そのエントリに Git ステータスのカラーが適用されます。

### Directory Flattening (`--flatten`)
通常の `ls` はカレントディレクトリ直下のエントリのみを表示します。`--flatten` を指定すると、サブディレクトリ内の変更（Modified や Untracked）を現在の階層に引き上げて一覧表示します：

``` console
$ glas --flatten           # 全階層の変更を再帰的に展開（--flatten=all と同等）
$ glas --flatten=2         # 最大2階層下まで展開
$ glas --flatten=all       # 再帰的にすべての階層の変更を展開
```

変更ファイルを指定した階層まで展開しない場合、ディレクトリごとにサマリー行（例: `src/ (12 modified files)`）として表示されます。

### Git Selectors (`--cached`, `--stage`, etc.)
`glas` は `git ls-files` と同様のフィルタリングオプションを提供します：
- `--cached`: インデックスで追跡中のファイルのみを表示。
- `--stage`: ファイルモード、オブジェクトハッシュ、ステージ番号を表示。
- `--modified`: 未ステージの変更または削除されたファイルを表示。
- `--deleted`: 削除されたファイルのみを表示。
- `--others`: 未追跡のファイルのみを表示。
- `--ignored`: 無視されたファイルのみを表示。
- `--include-ignored`: 通常の一覧に無視ファイルも含めて表示。

> [!NOTE]
> `--cached` や `--stage` は Git インデックス（ファイル単位）を対象とするため、自動的に `-f` (`--only-files`) が有効になり、ディレクトリは除外されます。
> セレクタはエントリごとの Git ステータスと一致するため、配下の変更を集約したディレクトリ行もこれらのセレクタに一致します。flatten の階層上限まで展開されない変更は、ディレクトリ行またはサマリー行として表示されます。

### Pipeline-Safe Output
`glas` は標準出力（stdout）の接続先を自動判別して出力を切り替えます：
- 端末（TTY）に接続されている場合は、カラー表示やグリッドレイアウト（複数列）で表示します。
- パイプやファイルリダイレクト時は、自動的にカラーとグリッドを無効化し、1行1エントリのプレーンなテキストを出力します。`--color=always` 指定時はパイプでもカラー出力されます。
- `-z` / `--null`: スペースや改行を含むファイル名を `xargs -0` 等で安全に扱うため、区切り文字を NULL 文字（`\0`）にします。
- `--format="%(size) %(path)"`: 必要なフィールドのみを直接指定して抽出できます。

### Non-Git Directory Fallback
Git リポジトリではないディレクトリで実行した場合でも、エラーや警告を出力することなく、通常のディレクトリ一覧ツール（グリッド表示、パーミッション、サイズ、ソート機能）として透過的に動作します。

## Shell Completion

`--completions` フラグで各シェルの補完スクリプトを生成できます：

``` console
$ glas --completions
```

対応シェル: `bash`, `elvish`, `fish`, `powershell`, `zsh`。

## Options

### DISPLAY OPTIONS
| Flag | Description |
| :--- | :--- |
| `-1`, `--oneline` | 1行に1エントリずつ表示する |
| `-l`, `--long` | 拡張メタデータを表形式で表示する |
| `-H`, `--header` | ロングフォーマット時にヘッダー行を表示する |
| `--color <COLOR>` | カラー出力を制御する (`always`, `auto`, `never`) [デフォルト: `auto`] |
| `--absolute` | エントリを絶対パスで表示する |
| `-z`, `--null` | 各行の末尾に NULL 文字（`\0`）を付加する |
| `--format <FORMAT>` | カスタム出力フォーマットを指定する (例: `%(size) %(path)`) |

### FILTERING AND SORTING OPTIONS
| Flag | Description |
| :--- | :--- |
| `-a`, `--all` | 隠しファイル（ドットファイル）を表示する |
| `-d`, `--treat-dirs-as-files` | ディレクトリをファイルとして一覧表示する（中身を展開しない） |
| `-D`, `--only-dirs` | ディレクトリのみを表示する |
| `-f`, `--only-files` | ファイルのみを表示する |
| `-s`, `--sort <SORT>` | 指定したフィールドでソートする (`name`, `size`, `time`, `git`) [デフォルト: `git`] |

### GIT-AWARE OPTIONS
| Flag | Description |
| :--- | :--- |
| `--cached` | インデックス内のキャッシュされたファイルを表示する (`-f` を暗黙に指定) |
| `--stage` | ステージング情報（モード、ハッシュ、ステージ番号）を表示する |
| `--deleted` | 未ステージの削除があるファイルを表示する |
| `--modified` | 未ステージの変更があるファイルを表示する |
| `--others` | その他の（未追跡の）ファイルを表示する |
| `--ignored` | 無視されたファイルのみを表示する |
| `--include-ignored` | 無視されたファイルも含めて表示する |
| `--no-git` | Git コンテキストの取得を無効にする |
| `--flatten[=<DEPTH>]` | ネストされた変更ファイルを指定階層まで展開して表示する (`0`, `1`, ..., `all`) [デフォルト: `0`] |
| `--full-name` | リポジトリルートからの相対パスで出力する |

## License

[MIT](LICENSE)

## Author

[Hagata](https://github.com/hagatasdelus)
