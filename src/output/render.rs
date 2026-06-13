//! # output/render
//!
//! Provides pre-rendering transformation logic for file entries, such as resolving absolute/relative paths
//! and sorting entries for output.

use std::cmp::Ordering;
use std::path::Path;
use std::time::SystemTime;

use crate::fs::file::{Entry, EntryKind};
use crate::fs::git::{GitContext, GitKind, StageInfo};
use crate::options::{RenderOptions, SortField};

/// Struct holding formatted information required for rendering.
#[derive(Debug)]
pub struct RenderedEntry {
    pub path: String,
    pub kind: EntryKind,
    pub git: GitKind,
    pub mode: Option<u32>,
    pub uid: Option<u32>,
    pub has_xattrs: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub stages: Vec<StageInfo>,
}

/// Formats the entry path as a string according to configuration options (e.g., converting to absolute paths,
/// or using paths relative to the Git repository root).
pub fn render_path(
    entry: &Entry,
    target_abs: &Path,
    git: Option<&GitContext>,
    options: &RenderOptions,
) -> String {
    let base_path = if options.absolute {
        entry.abs_path.to_string_lossy().into_owned()
    } else if options.full_name {
        if let Some(git) = git {
            if let Ok(path) = entry.abs_path.strip_prefix(&git.repo_root) {
                path.to_string_lossy().into_owned()
            } else {
                entry.rel_to_target.to_string_lossy().into_owned()
            }
        } else if let Ok(path) = entry.abs_path.strip_prefix(target_abs) {
            path.to_string_lossy().into_owned()
        } else {
            entry.rel_to_target.to_string_lossy().into_owned()
        }
    } else {
        entry.rel_to_target.to_string_lossy().into_owned()
    };

    if let EntryKind::Summary { modified_count } = entry.kind {
        let prefix = if base_path.ends_with('/') {
            base_path
        } else {
            format!("{base_path}/")
        };
        return format!("{prefix} ({modified_count} modified files)");
    }

    base_path
}

/// Sorts a slice of `RenderedEntry` based on the specified sort field (e.g., name, size, time, Git status).
pub fn sort_entries(entries: &mut [RenderedEntry], sort: Option<SortField>) {
    entries.sort_by(|left, right| match sort {
        Some(SortField::Name) => left.path.cmp(&right.path),
        Some(SortField::Size) => right
            .size
            .cmp(&left.size)
            .then_with(|| left.path.cmp(&right.path)),
        Some(SortField::Time) => {
            compare_modified(right.modified, left.modified).then_with(|| left.path.cmp(&right.path))
        }
        Some(SortField::Git) => left
            .git
            .rank()
            .cmp(&right.git.rank())
            .then_with(|| left.path.cmp(&right.path)),
        None => left
            .git
            .rank()
            .cmp(&right.git.rank())
            .then_with(|| left.path.cmp(&right.path)),
    });
}

fn compare_modified(left: Option<SystemTime>, right: Option<SystemTime>) -> Ordering {
    match (left, right) {
        (Some(l), Some(r)) => l.cmp(&r),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use crate::fs::git::GitKind;
    use std::time::SystemTime;

    fn dummy_entry_with_path(abs_path: &str, rel_path: &str) -> Entry {
        Entry {
            abs_path: std::path::PathBuf::from(abs_path),
            rel_to_target: std::path::PathBuf::from(rel_path),
            kind: EntryKind::File,
            git: GitKind::Clean,
            mode: 0o100644,
            uid: 0,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn test_render_path_absolute() {
        let entry = dummy_entry_with_path("/home/user/file.txt", "file.txt");
        let options = RenderOptions {
            long: false,
            header: false,
            null: false,
            format: None,
            absolute: true,
            full_name: false,
            sort: None,
            stage: false,
        };
        let rendered = render_path(&entry, Path::new("/home/user"), None, &options);
        assert_eq!(rendered, "/home/user/file.txt");
    }

    #[test]
    fn test_render_path_full_name_with_git() {
        let entry = dummy_entry_with_path("/repo/sub/file.txt", "file.txt");
        let git = GitContext {
            repo_root: std::path::PathBuf::from("/repo"),
            statuses: rustc_hash::FxHashMap::default(),
            stages: rustc_hash::FxHashMap::default(),
        };
        let options = RenderOptions {
            long: false,
            header: false,
            null: false,
            format: None,
            absolute: false,
            full_name: true,
            sort: None,
            stage: false,
        };
        let rendered = render_path(&entry, Path::new("/repo/sub"), Some(&git), &options);
        assert_eq!(rendered, "sub/file.txt");
    }

    #[test]
    fn test_render_path_full_name_without_git() {
        let entry = dummy_entry_with_path("/dir/sub/file.txt", "file.txt");
        let options = RenderOptions {
            long: false,
            header: false,
            null: false,
            format: None,
            absolute: false,
            full_name: true,
            sort: None,
            stage: false,
        };
        let rendered = render_path(&entry, Path::new("/dir"), None, &options);
        assert_eq!(rendered, "sub/file.txt");
    }

    #[test]
    fn test_render_path_summary() {
        let mut entry = dummy_entry_with_path("/dir/sub", "sub");
        entry.kind = EntryKind::Summary { modified_count: 3 };
        let options = RenderOptions {
            long: false,
            header: false,
            null: false,
            format: None,
            absolute: false,
            full_name: false,
            sort: None,
            stage: false,
        };
        let rendered = render_path(&entry, Path::new("/dir"), None, &options);
        assert_eq!(rendered, "sub/ (3 modified files)");
    }

    #[test]
    fn test_sort_entries_by_size() {
        let mut entries = vec![
            RenderedEntry {
                path: "small".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 10,
                modified: None,
                stages: Vec::new(),
            },
            RenderedEntry {
                path: "large".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 100,
                modified: None,
                stages: Vec::new(),
            },
            RenderedEntry {
                path: "medium".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 50,
                modified: None,
                stages: Vec::new(),
            },
        ];

        sort_entries(&mut entries, Some(SortField::Size));
        assert_eq!(entries[0].path, "large");
        assert_eq!(entries[1].path, "medium");
        assert_eq!(entries[2].path, "small");
    }

    #[test]
    fn test_sort_entries_by_time() {
        let t1 = Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(10));
        let t2 = Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(20));
        let mut entries = vec![
            RenderedEntry {
                path: "old".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 0,
                modified: t1,
                stages: Vec::new(),
            },
            RenderedEntry {
                path: "new".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 0,
                modified: t2,
                stages: Vec::new(),
            },
            RenderedEntry {
                path: "no_time".to_string(),
                kind: EntryKind::File,
                git: GitKind::Clean,
                mode: None,
                uid: None,
                has_xattrs: false,
                size: 0,
                modified: None,
                stages: Vec::new(),
            },
        ];

        sort_entries(&mut entries, Some(SortField::Time));
        assert_eq!(entries[0].path, "new");
        assert_eq!(entries[1].path, "old");
        assert_eq!(entries[2].path, "no_time");
    }
}
