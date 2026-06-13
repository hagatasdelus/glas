use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use crate::fs::git::{GitKind, StageInfo};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    File,
    Directory,
    Summary { modified_count: usize },
}

#[derive(Debug)]
pub struct Entry {
    pub abs_path: PathBuf,
    pub rel_to_target: PathBuf,
    pub kind: EntryKind,
    pub git: GitKind,
    pub mode: u32,
    pub uid: u32,
    pub has_xattrs: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
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
        let has_xattrs = include_xattrs && has_extended_attributes(&abs_path);

        Self {
            abs_path,
            rel_to_target,
            kind,
            git: GitKind::Clean,
            mode: metadata.mode(),
            uid: metadata.uid(),
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

pub fn has_extended_attributes(path: &Path) -> bool {
    match xattr::list(path) {
        Ok(mut attrs) => attrs.next().is_some(),
        Err(_) => false,
    }
}

pub fn absolutize(path: &Path) -> Result<PathBuf> {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("failed to get current directory")?
            .join(path)
    };

    match fs::canonicalize(&joined) {
        Ok(path) => Ok(path),
        Err(_) => Ok(joined),
    }
}

pub fn is_hidden_path(path: impl AsRef<Path>) -> bool {
    path.as_ref().components().any(|part| {
        if let Component::Normal(name) = part {
            name.to_string_lossy().starts_with('.')
        } else {
            false
        }
    })
}

pub fn component_to_path(component: Component<'_>) -> PathBuf {
    match component {
        Component::Normal(name) => PathBuf::from(name),
        _ => PathBuf::new(),
    }
}
