//! # options/config
//!
//! ディレクトリ探索時のフィルタリングや取得条件を設定する `DirOptions` と、
//! 出力時のフォーマットやソート条件などを設定する `RenderOptions` を定義します。

use crate::options::cli::{Cli, SortField};

/// フラット化の深さを定義する列挙型です。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlattenDepth {
    /// 指定された深さまでフラット化します。
    Depth(usize),
    /// すべての階層をフラット化します。
    All,
}

impl FlattenDepth {
    /// 文字列から `FlattenDepth` を構築します。
    /// "all" (大文字小文字無視) の場合は `FlattenDepth::All` になり、
    /// それ以外の場合は数値パース（失敗時は 0）を伴う `FlattenDepth::Depth` になります。
    pub fn from_str(s: &str) -> Self {
        if s.eq_ignore_ascii_case("all") {
            Self::All
        } else {
            Self::Depth(s.parse().unwrap_or(0))
        }
    }
}

/// ディレクトリ探索時の振る舞いを決定するオプション設定です。
#[derive(Clone, Debug)]
pub struct DirOptions {
    /// Git 連携を完全に無効化するかどうか。
    pub no_git: bool,
    /// ディレクトリ自体をファイルのように扱うかどうか。
    pub treat_dirs_as_files: bool,
    /// 詳細表示モードかどうか。
    pub long: bool,
    /// 隠しファイルを含めるかどうか。
    pub all: bool,
    /// ディレクトリのみを表示するかどうか。
    pub only_dirs: bool,
    /// ファイルのみを表示するかどうか。
    pub only_files: bool,

    // Git-aware options
    /// インデックスに登録されている（キャッシュされた）ファイルのみを表示するかどうか。
    pub cached: bool,
    /// ステージ情報を含めるかどうか。
    pub stage: bool,
    /// 削除されたファイルのみを表示するかどうか。
    pub deleted: bool,
    /// 変更されたファイルのみを表示するかどうか。
    pub modified: bool,
    /// 未追跡のファイルのみを表示するかどうか。
    pub others: bool,
    /// 無視されたファイルのみを表示するかどうか。
    pub ignored: bool,
    /// 表示対象に無視ファイルも含めるかどうか。
    pub include_ignored: bool,
    /// ディレクトリ構造のフラット化の設定です。
    pub flatten: FlattenDepth,
    /// Git系の特定の選択フィルタ（cached/stage/deleted/modified/others/ignored）が一つでも有効になっているかどうか。
    pub git_select_mode: bool,
}

impl DirOptions {
    /// パース済みのコマンドライン引数（`Cli`）から `DirOptions` を生成します。
    pub fn from_cli(cli: &Cli) -> Self {
        let cached = cli.cached;
        let stage = cli.stage;
        let deleted = cli.deleted;
        let modified = cli.modified;
        let others = cli.others;
        let ignored = cli.ignored;

        let git_select_mode = cached || stage || deleted || modified || others || ignored;

        Self {
            no_git: cli.no_git,
            treat_dirs_as_files: cli.treat_dirs_as_files,
            long: cli.long,
            all: cli.all,
            only_dirs: cli.only_dirs,
            only_files: cli.only_files,
            cached,
            stage,
            deleted,
            modified,
            others,
            ignored,
            include_ignored: cli.include_ignored,
            flatten: FlattenDepth::from_str(&cli.flatten),
            git_select_mode,
        }
    }
}

/// 出力結果のレンダリングを制御するオプション設定です。
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// テーブル形式での詳細表示（パーミッション、サイズ等）を行うかどうか。
    pub long: bool,
    /// ヘッダー行を表示するかどうか。
    pub header: bool,
    /// 出力を NUL 文字で区切るかどうか。
    pub null: bool,
    /// 独自のカスタム出力フォーマットテンプレート。
    pub format: Option<String>,
    /// 絶対パスで表示するかどうか。
    pub absolute: bool,
    /// Git リポジトリルートからの相対パスで表示するかどうか。
    pub full_name: bool,
    /// ソート条件です。
    pub sort: Option<SortField>,
    /// ステージ情報を表示するかどうか。
    pub stage: bool,
}

impl RenderOptions {
    /// コマンドライン引数（`Cli`）から `RenderOptions` を生成します。
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            long: cli.long,
            header: cli.header,
            null: cli.null,
            format: cli.format.clone(),
            absolute: cli.absolute,
            full_name: cli.full_name,
            sort: cli.sort,
            stage: cli.stage,
        }
    }
}
