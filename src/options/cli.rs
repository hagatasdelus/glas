use clap::{Parser, ValueEnum};
use std::path::PathBuf;

const HELP_TEMPLATE: &str = "USAGE:\n  {usage}\n\n{all-args}";

#[derive(Debug, Parser)]
#[command(name = "glas")]
#[command(version = include_str!(concat!(env!("OUT_DIR"), "/version_string.txt")))]
#[command(override_usage = "glas [OPTIONS] [FILE]...")]
#[command(help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[arg(
        short = '1',
        long = "oneline",
        help = "display one entry per line",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub oneline: bool,

    #[arg(
        short = 'l',
        long = "long",
        help = "display extended file metadata as a table",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub long: bool,

    #[arg(
        short = 'H',
        long = "header",
        help = "display a header row in long format",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub header: bool,

    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto, help_heading = "DISPLAY OPTIONS")]
    pub color: ColorWhen,

    #[arg(
        long = "absolute",
        help = "display entries with their absolute paths",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub absolute: bool,

    #[arg(
        short = 'z',
        long = "null",
        help = "terminate lines with a null byte",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub null: bool,

    #[arg(
        long = "format",
        help = "custom output format",
        help_heading = "DISPLAY OPTIONS"
    )]
    pub format: Option<String>,

    #[arg(
        short = 'a',
        long = "all",
        help = "show hidden and dot-files",
        help_heading = "FILTERING AND SORTING OPTIONS"
    )]
    pub all: bool,

    #[arg(
        short = 'd',
        long = "treat-dirs-as-files",
        help = "list directories as files",
        help_heading = "FILTERING AND SORTING OPTIONS"
    )]
    pub treat_dirs_as_files: bool,

    #[arg(
        short = 'D',
        long = "only-dirs",
        help = "list only directories",
        help_heading = "FILTERING AND SORTING OPTIONS"
    )]
    pub only_dirs: bool,

    #[arg(
        short = 'f',
        long = "only-files",
        help = "list only files",
        help_heading = "FILTERING AND SORTING OPTIONS"
    )]
    pub only_files: bool,

    #[arg(
        short = 's',
        long = "sort",
        value_enum,
        help_heading = "FILTERING AND SORTING OPTIONS"
    )]
    pub sort: Option<SortField>,

    #[arg(
        long = "cached",
        help = "show cached files in index (default)",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub cached: bool,

    #[arg(
        long = "stage",
        alias = "staged",
        help = "show staged contents' mode bits, object name and stage number",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub stage: bool,

    #[arg(
        long = "deleted",
        help = "show files with an unstaged deletion",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub deleted: bool,

    #[arg(
        long = "modified",
        help = "show files with an unstaged modification",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub modified: bool,

    #[arg(
        long = "others",
        alias = "other",
        help = "show other (untracked) files in the output",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub others: bool,

    #[arg(
        long = "ignored",
        help = "show only ignored files in the output",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub ignored: bool,

    #[arg(
        long = "include-ignored",
        alias = "show-ignored",
        help = "include ignored files in output",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub include_ignored: bool,

    #[arg(
        long = "no-git",
        help = "disable Git context fetching",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub no_git: bool,

    #[arg(
        long = "flatten",
        num_args = 0..=1,
        require_equals = true,
        default_value = "0",
        default_missing_value = "all",
        help = "flatten nested modified files",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub flatten: String,

    #[arg(
        long = "full-name",
        help = "output paths relative to repo root",
        help_heading = "GIT-AWARE OPTIONS"
    )]
    pub full_name: bool,

    #[arg(long, help = "generate completion files", default_value_t = false)]
    pub completions: bool,

    #[arg(value_name = "FILE", hide = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ColorWhen {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SortField {
    Name,
    Size,
    Time,
    Git,
}
