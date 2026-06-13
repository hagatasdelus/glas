use std::cmp::Ordering;
use std::path::Path;
use std::time::SystemTime;

use crate::fs::file::{Entry, EntryKind};
use crate::fs::git::{GitContext, GitKind, StageInfo};
use crate::options::{RenderOptions, SortField};

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
