//! # fs/git
//!
//! Git リポジトリのステータス情報（追跡状況、変更ステージ、競合など）の取得と
//! ファイルエントリーへのマッピング処理を提供するモジュールです。

use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};
use rustc_hash::{FxHashMap, FxHashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::fs::file::{Entry, EntryKind, component_to_path, is_hidden_path};
use crate::options::{DirOptions, config::FlattenDepth};

/// ファイルの Git ステータスの種類を定義する列挙型です。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GitKind {
    /// 競合状態。
    Conflicted,
    /// ステージング（索引に追加）された状態。
    Staged,
    /// 変更されている（ステージング未登録）状態。
    Modified,
    /// 削除された状態。
    Deleted,
    /// 未追跡（新規作成かつ未登録）の状態。
    Untracked,
    /// 無視リスト（.gitignore等）に登録されている状態。
    Ignored,
    /// 変更のないクリーンな状態。
    Clean,
}

impl GitKind {
    pub fn from_status(status: Status) -> Self {
        if status.is_conflicted() {
            return Self::Conflicted;
        }

        if status.intersects(
            Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::INDEX_RENAMED
                | Status::INDEX_TYPECHANGE,
        ) {
            return Self::Staged;
        }

        if status.intersects(Status::WT_DELETED) {
            return Self::Deleted;
        }

        if status.intersects(Status::WT_MODIFIED | Status::WT_RENAMED | Status::WT_TYPECHANGE) {
            return Self::Modified;
        }

        if status.is_wt_new() {
            return Self::Untracked;
        }

        if status.is_ignored() {
            return Self::Ignored;
        }

        Self::Clean
    }

    pub fn rank(self) -> u8 {
        match self {
            Self::Conflicted => 0,
            Self::Staged => 1,
            Self::Deleted => 2,
            Self::Modified => 3,
            Self::Untracked => 4,
            Self::Ignored => 5,
            Self::Clean => 6,
        }
    }

    pub fn merge(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    pub fn tag(self) -> &'static str {
        match self {
            Self::Conflicted => "!",
            Self::Staged => "S",
            Self::Modified => "M",
            Self::Deleted => "D",
            Self::Untracked => "?",
            Self::Ignored => "I",
            Self::Clean => " ",
        }
    }
}

#[derive(Clone, Debug)]
/// Git インデックス上のステージ情報を保持する構造体です。
pub struct StageInfo {
    /// ファイルのパーミッションモード。
    pub mode: u32,
    /// Gitオブジェクトハッシュ。
    pub object_id: String,
    /// ステージ番号（1, 2, 3等）。
    pub stage: i32,
}

/// 対象ディレクトリにおける Git リポジトリの各種コンテキスト情報を保持する構造体です。
#[derive(Debug)]
pub struct GitContext {
    /// Git リポジトリのワークツリーのルート絶対パス。
    pub repo_root: PathBuf,
    /// 各ファイルパスと Git ステータス（`GitKind`）のマッピング。
    pub statuses: FxHashMap<PathBuf, GitKind>,
    /// 各ファイルパスと Git ステージ情報のリストのマッピング。
    pub stages: FxHashMap<PathBuf, Vec<StageInfo>>,
}

/// 指定された絶対パスから Git リポジトリを検出し、そのリポジトリ上の
/// コンテキスト情報（変更ステータス、インデックス等）をロードします。
/// Git リポジトリでない場合は `Ok(None)` を返します。
pub fn load_git_context(target_abs: &Path, show_ignored: bool) -> Result<Option<GitContext>> {
    let repo = match Repository::discover(target_abs) {
        Ok(repo) => repo,
        Err(_) => return Ok(None),
    };

    let repo_root = match repo.workdir() {
        Some(path) => path.to_path_buf(),
        None => return Ok(None),
    };

    let mut status_map: FxHashMap<PathBuf, GitKind> = FxHashMap::default();
    let mut stage_map: FxHashMap<PathBuf, Vec<StageInfo>> = FxHashMap::default();

    if let Ok(index) = repo.index() {
        for entry in index.iter() {
            let path_str = String::from_utf8_lossy(&entry.path);
            let repo_rel = PathBuf::from(path_str.as_ref());
            let abs_path = repo_root.join(&repo_rel);
            let stage = ((entry.flags & 0x3000) >> 12) as i32;
            let info = StageInfo {
                mode: entry.mode,
                object_id: format!("{}", entry.id),
                stage,
            };
            stage_map.entry(abs_path).or_default().push(info);
            status_map.insert(repo_rel, GitKind::Clean);
        }
    }

    let mut options = StatusOptions::new();
    options
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(show_ignored)
        .renames_head_to_index(true)
        .renames_index_to_workdir(true)
        .include_unmodified(false);

    let statuses = repo
        .statuses(Some(&mut options))
        .context("failed to collect git statuses")?;

    for status in statuses.iter() {
        let Some(path) = status.path() else {
            continue;
        };
        let kind = GitKind::from_status(status.status());
        if kind == GitKind::Clean {
            continue;
        }
        let key = PathBuf::from(path);
        if let Some(existing) = status_map.get(&key).copied() {
            status_map.insert(key, existing.merge(kind));
        } else {
            status_map.insert(key, kind);
        }
    }

    Ok(Some(GitContext {
        repo_root,
        statuses: status_map,
        stages: stage_map,
    }))
}

/// 収集されたファイルエントリー群に対し、Git コンテキスト情報（ステータスやステージ）
/// を適用・統合し、Gitのフィルタリング条件やフラット化設定に基づいてエントリーリストを再構成します。
pub fn apply_git_overlay(
    entries: &mut Vec<Entry>,
    target_abs: &Path,
    git: &GitContext,
    options: &DirOptions,
) -> Result<()> {
    let mut seen_paths: FxHashSet<PathBuf> =
        entries.iter().map(|entry| entry.abs_path.clone()).collect();
    let mut by_abs: FxHashMap<PathBuf, usize> = entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| (entry.abs_path.clone(), idx))
        .collect();
    let mut summary_counts: FxHashMap<PathBuf, usize> = FxHashMap::default();

    // First merge stage info for existing entries loaded from disk
    for entry in entries.iter_mut() {
        if let Some(stages) = git.stages.get(&entry.abs_path) {
            entry.stages = stages.clone();
        }
    }

    for (repo_rel, git_kind) in &git.statuses {
        let abs = git.repo_root.join(repo_rel);
        if !abs.starts_with(target_abs) {
            continue;
        }

        let rel = match abs.strip_prefix(target_abs) {
            Ok(path) => path,
            _ => continue,
        };

        if rel.as_os_str().is_empty() {
            if let Some(idx) = by_abs.get(&abs).copied() {
                entries[idx].git = entries[idx].git.merge(*git_kind);
                if let Some(stages) = git.stages.get(&abs) {
                    entries[idx].stages = stages.clone();
                }
            } else if *git_kind == GitKind::Deleted {
                let mut entry = Entry::new_deleted(abs.clone(), rel.to_path_buf());
                if let Some(stages) = git.stages.get(&abs) {
                    entry.stages = stages.clone();
                }
                by_abs.insert(abs.clone(), entries.len());
                seen_paths.insert(abs.clone());
                entries.push(entry);
            }
            continue;
        }

        let depth = rel.components().count();

        if let Some(first) = rel.components().next() {
            let top_abs = target_abs.join(component_to_path(first));
            if let Some(top_idx) = by_abs.get(&top_abs).copied() {
                entries[top_idx].git = entries[top_idx].git.merge(*git_kind);
            }
        }

        if depth <= 1 {
            if let Some(idx) = by_abs.get(&abs).copied() {
                entries[idx].git = entries[idx].git.merge(*git_kind);
                if let Some(stages) = git.stages.get(&abs) {
                    entries[idx].stages = stages.clone();
                }
            } else if *git_kind == GitKind::Deleted {
                let mut entry = Entry::new_deleted(abs.clone(), rel.to_path_buf());
                if let Some(stages) = git.stages.get(&abs) {
                    entry.stages = stages.clone();
                }
                by_abs.insert(abs.clone(), entries.len());
                seen_paths.insert(abs.clone());
                entries.push(entry);
            }
            continue;
        }

        let should_flatten = match options.flatten {
            FlattenDepth::All => true,
            FlattenDepth::Depth(d) => depth <= d.saturating_add(1),
        };

        if should_flatten {
            if !options.all && is_hidden_path(rel) {
                continue;
            }
            if seen_paths.contains(&abs) {
                continue;
            }

            let metadata = match fs::symlink_metadata(&abs) {
                Ok(m) => m,
                Err(_) => {
                    if *git_kind == GitKind::Deleted {
                        let mut entry = Entry::new_deleted(abs.clone(), rel.to_path_buf());
                        if let Some(stages) = git.stages.get(&abs) {
                            entry.stages = stages.clone();
                        }
                        by_abs.insert(abs.clone(), entries.len());
                        seen_paths.insert(abs);
                        entries.push(entry);
                    }
                    continue;
                }
            };
            let mut entry =
                Entry::new_file_or_dir(abs.clone(), rel.to_path_buf(), metadata, options.long);
            entry.git = *git_kind;
            if let Some(stages) = git.stages.get(&abs) {
                entry.stages = stages.clone();
            }
            by_abs.insert(abs.clone(), entries.len());
            seen_paths.insert(abs);
            entries.push(entry);
            continue;
        }

        if let Some(first) = rel.components().next() {
            let top_abs = target_abs.join(component_to_path(first));
            if !options.all && is_hidden_path(component_to_path(first)) {
                continue;
            }
            *summary_counts.entry(top_abs).or_insert(0) += 1;
        }
    }

    for entry in entries.iter_mut() {
        if !matches!(entry.kind, EntryKind::Directory) || !matches!(entry.git, GitKind::Clean) {
            continue;
        }

        let mut dir_rel = entry.rel_to_target.clone();
        dir_rel.push("");
        if git.statuses.keys().any(|repo_rel| {
            git.repo_root
                .join(repo_rel)
                .starts_with(target_abs.join(&dir_rel))
        }) {
            entry.git = GitKind::Modified;
        }
    }

    for (top_abs, count) in summary_counts {
        if seen_paths.contains(&top_abs) {
            continue;
        }
        let rel = match top_abs.strip_prefix(target_abs) {
            Ok(path) => path.to_path_buf(),
            Err(_) => continue,
        };
        entries.push(Entry {
            abs_path: top_abs.clone(),
            rel_to_target: rel,
            kind: EntryKind::Summary {
                modified_count: count,
            },
            git: GitKind::Modified,
            mode: 0,
            uid: 0,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        });
        seen_paths.insert(top_abs);
    }

    Ok(())
}
