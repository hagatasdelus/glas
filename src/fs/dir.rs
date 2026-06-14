//! # fs/dir
//!
//! Provides directory traversal functionality and collects file entries matching the `DirOptions` criteria.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use crate::fs::file::{Entry, EntryKind, absolutize, is_hidden_path};
use crate::fs::git::{GitKind, apply_git_overlay, load_git_context};
use crate::options::{DirOptions, RenderOptions};
use crate::output::render::{RenderedEntry, render_path};

fn matches_git_selectors(kind: GitKind, options: &DirOptions, has_git: bool) -> bool {
    if !has_git {
        if options.ignored {
            return false;
        }
        if options.git_select_mode
            && (options.deleted || options.modified)
            && !options.cached
            && !options.others
            && !options.stage
        {
            return false;
        }
        return true;
    }

    if !options.git_select_mode {
        if kind == GitKind::Ignored {
            return options.include_ignored;
        }
        if kind == GitKind::Deleted {
            return false;
        }
        return true;
    }

    if options.ignored {
        return kind == GitKind::Ignored;
    }

    let mut matched = false;

    let is_cached_active = options.cached
        || (options.stage && !options.deleted && !options.modified && !options.others);

    if is_cached_active {
        matched |= matches!(
            kind,
            GitKind::Clean
                | GitKind::Staged
                | GitKind::Modified
                | GitKind::Deleted
                | GitKind::Conflicted
        );
    }
    if options.deleted {
        matched |= matches!(kind, GitKind::Deleted);
    }
    if options.modified {
        matched |= matches!(kind, GitKind::Modified | GitKind::Deleted);
    }
    if options.others {
        matched |= matches!(kind, GitKind::Untracked);
    }
    if options.include_ignored {
        matched |= matches!(kind, GitKind::Ignored);
    }

    matched
}

/// Traverses file and directory entries under the specified target path
/// and collects those that match filters into a vector of `RenderedEntry`.
pub fn collect_target_entries(
    target: &Path,
    dir_options: &DirOptions,
    render_options: &RenderOptions,
) -> Result<Vec<RenderedEntry>> {
    let target_abs = absolutize(target)?;
    let target_meta = fs::symlink_metadata(&target_abs)
        .with_context(|| format!("failed to read metadata for {}", target_abs.display()))?;

    let git_handle = if dir_options.no_git {
        None
    } else {
        let target_for_thread = target_abs.clone();
        let show_ignored = dir_options.include_ignored || dir_options.ignored;
        Some(thread::spawn(move || {
            load_git_context(&target_for_thread, show_ignored)
        }))
    };

    let mut entries = if target_meta.is_dir() && !dir_options.treat_dirs_as_files {
        collect_directory_entries(&target_abs, dir_options)?
    } else {
        let rel = target_abs
            .file_name()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        vec![Entry::new_file_or_dir(
            target_abs.clone(),
            rel,
            target_meta,
            dir_options.long,
        )]
    };

    let git_context = if let Some(handle) = git_handle {
        handle
            .join()
            .map_err(|_| anyhow::anyhow!("failed to join git status worker"))??
    } else {
        None
    };

    if let Some(git) = git_context.as_ref() {
        apply_git_overlay(&mut entries, &target_abs, git, dir_options)?;
    }

    let has_git = git_context.is_some();
    entries.retain(|entry| {
        let kind_ok = match entry.kind {
            EntryKind::File => !dir_options.only_dirs,
            EntryKind::Directory | EntryKind::Summary { .. } => !dir_options.only_files,
        };
        if !kind_ok {
            return false;
        }

        matches_git_selectors(entry.git, dir_options, has_git)
    });

    let mut rendered = Vec::with_capacity(entries.len());
    for entry in entries {
        rendered.push(RenderedEntry {
            path: render_path(&entry, &target_abs, git_context.as_ref(), render_options),
            kind: entry.kind,
            git: entry.git,
            mode: entry.display_mode(),
            uid: entry.display_uid(),
            has_xattrs: entry.has_xattrs,
            size: entry.size,
            modified: entry.modified,
            stages: entry.stages.clone(),
        });
    }

    Ok(rendered)
}

fn collect_directory_entries(target_abs: &Path, dir_options: &DirOptions) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    let dir_iter = fs::read_dir(target_abs)
        .with_context(|| format!("failed to read {}", target_abs.display()))?;

    for item in dir_iter {
        let item = item.with_context(|| format!("failed to read {}", target_abs.display()))?;
        let rel = PathBuf::from(item.file_name());

        if !dir_options.all && is_hidden_path(&rel) {
            continue;
        }

        let item_path = item.path();
        let metadata = fs::symlink_metadata(&item_path)
            .with_context(|| format!("failed to read metadata for {}", item_path.display()))?;
        let entry = Entry::new_file_or_dir(item_path, rel, metadata, dir_options.long);
        if dir_options.only_dirs && !matches!(entry.kind, EntryKind::Directory) {
            continue;
        }
        if dir_options.only_files && !matches!(entry.kind, EntryKind::File) {
            continue;
        }
        entries.push(entry);
    }

    Ok(entries)
}
