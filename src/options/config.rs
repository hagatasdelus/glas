//! # options/config
//!
//! Defines `DirOptions` for filtering and traversal during directory scanning,
//! and `RenderOptions` for output formatting and sorting configuration.

use crate::options::cli::{Cli, SortField};

/// An enum defining the depth of directory flattening.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlattenDepth {
    /// Flatten up to the specified depth.
    Depth(usize),
    /// Flatten all nested levels.
    All,
}

impl FlattenDepth {
    /// Constructs a `FlattenDepth` from a string.
    /// Case-insensitively resolves "all" to `FlattenDepth::All`,
    /// and parses numeric values (defaulting to 0 on failure) for `FlattenDepth::Depth`.
    pub fn from_str(s: &str) -> Self {
        if s.eq_ignore_ascii_case("all") {
            Self::All
        } else {
            Self::Depth(s.parse().unwrap_or(0))
        }
    }
}

/// Configuration options for directory traversal and entry filtering.
#[derive(Clone, Debug)]
pub struct DirOptions {
    /// Whether to completely disable Git integration.
    pub no_git: bool,
    /// Whether to list directories as if they were regular files.
    pub treat_dirs_as_files: bool,
    /// Whether detailed (long) output mode is active.
    pub long: bool,
    /// Whether to include hidden files.
    pub all: bool,
    /// Whether to list only directories.
    pub only_dirs: bool,
    /// Whether to list only files.
    pub only_files: bool,

    // Git-aware options
    /// Whether to show only cached (staged) files.
    pub cached: bool,
    /// Whether to include stage information.
    pub stage: bool,
    /// Whether to show only deleted files.
    pub deleted: bool,
    /// Whether to show only modified files.
    pub modified: bool,
    /// Whether to show only untracked files.
    pub others: bool,
    /// Whether to show only ignored files.
    pub ignored: bool,
    /// Whether to include ignored files in output filters.
    pub include_ignored: bool,
    /// Flattening depth configuration.
    pub flatten: FlattenDepth,
    /// Whether any specific Git-related selector filter is enabled.
    pub git_select_mode: bool,
}

impl DirOptions {
    /// Generates `DirOptions` from parsed command-line arguments (`Cli`).
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

/// Options controlling output rendering and formatting.
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// Whether to use the detailed long format (table mode).
    pub long: bool,
    /// Whether to display a header row.
    pub header: bool,
    /// Whether to separate outputs using NUL character.
    pub null: bool,
    /// Custom template for rendering output entries.
    pub format: Option<String>,
    /// Whether to output absolute paths.
    pub absolute: bool,
    /// Whether to display paths relative to the Git repository root.
    pub full_name: bool,
    /// Sort criteria.
    pub sort: Option<SortField>,
    /// Whether to display stage information.
    pub stage: bool,
}

impl RenderOptions {
    /// Generates `RenderOptions` from command-line arguments (`Cli`).
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
