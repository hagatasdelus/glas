use crate::options::cli::{Cli, SortField};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlattenDepth {
    Depth(usize),
    All,
}

impl FlattenDepth {
    pub fn from_str(s: &str) -> Self {
        if s.eq_ignore_ascii_case("all") {
            Self::All
        } else {
            Self::Depth(s.parse().unwrap_or(0))
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirOptions {
    pub no_git: bool,
    pub treat_dirs_as_files: bool,
    pub long: bool,
    pub all: bool,
    pub only_dirs: bool,
    pub only_files: bool,

    // Git-aware options
    pub cached: bool,
    pub stage: bool,
    pub deleted: bool,
    pub modified: bool,
    pub others: bool,
    pub ignored: bool,
    pub include_ignored: bool,
    pub flatten: FlattenDepth,
    pub git_select_mode: bool,
}

impl DirOptions {
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

#[derive(Clone, Debug)]
pub struct RenderOptions {
    pub long: bool,
    pub header: bool,
    pub null: bool,
    pub format: Option<String>,
    pub absolute: bool,
    pub full_name: bool,
    pub sort: Option<SortField>,
    pub stage: bool,
}

impl RenderOptions {
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
