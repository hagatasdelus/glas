//! # fs/file
//!
//! Defines the `Entry` struct representing file/directory metadata
//! and provides path manipulation helper functions.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use crate::fs::git::{GitKind, StageInfo};

/// Enum defining types of entries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    /// A regular file.
    File,
    /// A directory.
    Directory,
    /// A summary of modified files.
    Summary { modified_count: usize },
}

/// Struct holding various metadata of a file or directory (permissions, owner, size, timestamp, etc.).
#[derive(Debug)]
pub struct Entry {
    /// Absolute path.
    pub abs_path: PathBuf,
    /// Relative path from the traversal target directory.
    pub rel_to_target: PathBuf,
    /// The entry type.
    pub kind: EntryKind,
    /// Git status information.
    pub git: GitKind,
    /// File permission mode bits.
    pub mode: u32,
    /// Owner's user ID (UID).
    pub uid: u32,
    /// Whether the file has extended attributes (xattr).
    pub has_xattrs: bool,
    /// File size in bytes.
    pub size: u64,
    /// Last modification time.
    pub modified: Option<SystemTime>,
    /// Git staging details.
    pub stages: Vec<StageInfo>,
}

impl Entry {
    pub fn new_file_or_dir(
        abs_path: PathBuf,
        rel_to_target: PathBuf,
        metadata: fs::Metadata,
        include_xattrs: bool,
    ) -> Self {
        let kind = if metadata.is_dir() {
            EntryKind::Directory
        } else {
            EntryKind::File
        };

        #[cfg(unix)]
        let mode = metadata.mode();
        #[cfg(not(unix))]
        let mode = if metadata.is_dir() { 0o40755 } else { 0o100644 };

        #[cfg(unix)]
        let uid = metadata.uid();
        #[cfg(not(unix))]
        let uid = 0;

        let has_xattrs = include_xattrs && has_extended_attributes(&abs_path);

        Self {
            abs_path,
            rel_to_target,
            kind,
            git: GitKind::Clean,
            mode,
            uid,
            has_xattrs,
            size: metadata.len(),
            modified: metadata.modified().ok(),
            stages: Vec::new(),
        }
    }

    pub fn new_deleted(abs_path: PathBuf, rel_to_target: PathBuf) -> Self {
        Self {
            abs_path,
            rel_to_target,
            kind: EntryKind::File,
            git: GitKind::Deleted,
            mode: 0o100644,
            uid: 0,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    pub fn display_mode(&self) -> Option<u32> {
        match self.kind {
            EntryKind::Summary { .. } => None,
            _ => Some(self.mode),
        }
    }

    pub fn display_uid(&self) -> Option<u32> {
        match self.kind {
            EntryKind::Summary { .. } => None,
            _ => Some(self.uid),
        }
    }
}

#[cfg(unix)]
pub fn has_extended_attributes(path: &Path) -> bool {
    match xattr::list(path) {
        Ok(mut attrs) => attrs.next().is_some(),
        Err(_) => false,
    }
}

#[cfg(not(unix))]
pub fn has_extended_attributes(_path: &Path) -> bool {
    false
}

/// Resolves a path to an absolute path. If resolution fails or path is non-existent,
/// falls back to joining with the current directory.
pub fn absolutize(path: &Path) -> Result<PathBuf> {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("failed to get current directory")?
            .join(path)
    };

    match dunce::canonicalize(&joined) {
        Ok(path) => Ok(path),
        Err(_) => Ok(joined),
    }
}

/// Determines whether the path contains any hidden components starting with a dot `.`.
pub fn is_hidden_path(path: impl AsRef<Path>) -> bool {
    path.as_ref().components().any(|part| {
        if let Component::Normal(name) = part {
            name.to_string_lossy().starts_with('.')
        } else {
            false
        }
    })
}

/// Converts a path component to a `PathBuf`. Returns an empty path for special components.
pub fn component_to_path(component: Component<'_>) -> PathBuf {
    match component {
        Component::Normal(name) => PathBuf::from(name),
        _ => PathBuf::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_display_methods() {
        let entry_file = Entry::new_deleted(PathBuf::from("/a"), PathBuf::from("a"));
        assert_eq!(entry_file.display_mode(), Some(0o100644));
        assert_eq!(entry_file.display_uid(), Some(0));

        let entry_summary = Entry {
            abs_path: PathBuf::from("/a"),
            rel_to_target: PathBuf::from("a"),
            kind: EntryKind::Summary { modified_count: 5 },
            git: GitKind::Clean,
            mode: 0o100644,
            uid: 0,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        };
        assert_eq!(entry_summary.display_mode(), None);
        assert_eq!(entry_summary.display_uid(), None);
    }

    #[test]
    fn test_absolutize() {
        let abs = Path::new("/foo/bar");
        assert_eq!(absolutize(abs).unwrap(), PathBuf::from("/foo/bar"));

        let rel = Path::new("nonexistent_file_xyz");
        let abs_rel = absolutize(rel).unwrap();
        assert!(abs_rel.is_absolute());
        assert!(abs_rel.ends_with("nonexistent_file_xyz"));
    }

    #[test]
    fn test_is_hidden_path() {
        assert!(is_hidden_path(".hidden"));
        assert!(is_hidden_path("dir/.hidden"));
        assert!(is_hidden_path("dir/.hidden/file"));
        assert!(!is_hidden_path("visible"));
        assert!(!is_hidden_path("dir/visible/file"));
    }

    #[test]
    fn test_component_to_path() {
        assert_eq!(
            component_to_path(Component::Normal(std::ffi::OsStr::new("foo"))),
            PathBuf::from("foo")
        );
        assert_eq!(component_to_path(Component::RootDir), PathBuf::new());
        assert_eq!(component_to_path(Component::CurDir), PathBuf::new());
    }
}
