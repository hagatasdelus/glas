//! # theme/color
//!
//! ファイルエントリーの種類（ディレクトリ）や Git のステータスに応じて
//! 適切な ANSI カラー表示や太字表示を適用する処理を提供するモジュールです。

use owo_colors::OwoColorize;

use crate::fs::git::GitKind;
use crate::output::render::RenderedEntry;

/// 指定されたファイルパスのレンダリング文字列に、Git ステータスやファイル種類、
/// 特殊ファイルのルールに基づいて ANSI エスケープのカラーやスタイル（太字）を適用します。
pub fn apply_color(
    rendered: &str,
    entry: &RenderedEntry,
    color_enabled: bool,
    _long: bool,
) -> String {
    if !color_enabled {
        return rendered.to_string();
    }

    let is_special_file = entry.path == "Cargo.toml" || entry.path == "justfile";

    match entry.git {
        GitKind::Conflicted => rendered.red().to_string(),
        GitKind::Staged => rendered.green().to_string(),
        GitKind::Modified => rendered.yellow().to_string(),
        GitKind::Deleted => rendered.red().to_string(),
        GitKind::Untracked => rendered.cyan().to_string(),
        GitKind::Ignored => rendered.bright_black().to_string(),
        GitKind::Clean => {
            if entry.kind == crate::fs::file::EntryKind::Directory {
                rendered.blue().to_string()
            } else if is_special_file {
                format!("\u{1b}[1m{}\u{1b}[0m", rendered)
            } else {
                rendered.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use owo_colors::OwoColorize;

    fn dummy_entry_with_git(git: GitKind) -> RenderedEntry {
        RenderedEntry {
            path: "test".to_string(),
            kind: EntryKind::File,
            git,
            mode: Some(0o100644),
            uid: Some(0),
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn test_apply_color_disabled() {
        let entry = dummy_entry_with_git(GitKind::Modified);
        assert_eq!(apply_color("hello", &entry, false, false), "hello");
    }

    #[test]
    fn test_apply_color_git_kinds() {
        let entry = dummy_entry_with_git(GitKind::Conflicted);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".red().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Staged);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".green().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Deleted);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".red().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Modified);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".yellow().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Untracked);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".cyan().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Ignored);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".bright_black().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Clean);
        assert_eq!(apply_color("hello", &entry, true, false), "hello");
    }
}
