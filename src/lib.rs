//! # glas
//!
//! `glas` is a Git-status-aware modern replacement for the `ls` command.
//! It displays file permissions, size, modification time, and Git status in a colorful list.

mod fs;
mod gencomp;
mod info;
mod options;
mod output;
mod theme;

use anyhow::Result;
use clap::Parser;
use std::io::IsTerminal;
use std::path::PathBuf;

use fs::collect_target_entries;
use options::{Cli, ColorWhen, DirOptions, RenderOptions, resolve_layout_mode};
use output::{sort_entries, write_output};

/// The main entry point of the `glas` command.
/// Parses command-line arguments and displays the contents of the specified directories.
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.completions {
        gencomp::generate(std::path::Path::new("completions"));
        return Ok(());
    }
    run_with_cli(cli)
}

#[derive(Debug)]
pub struct PartialFailure {
    pub errors: Vec<(PathBuf, anyhow::Error)>,
}

impl std::fmt::Display for PartialFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Partial failure during target scanning ({} errors)",
            self.errors.len()
        )
    }
}

impl std::error::Error for PartialFailure {}

fn run_with_cli(cli: Cli) -> Result<()> {
    let stdout_is_tty = std::io::stdout().is_terminal();
    let layout = resolve_layout_mode(&cli, stdout_is_tty);

    let color_enabled = match cli.color {
        ColorWhen::Always => true,
        ColorWhen::Never => false,
        ColorWhen::Auto => stdout_is_tty,
    };

    let dir_options = DirOptions::from_cli(&cli);
    let render_options = RenderOptions::from_cli(&cli);

    let targets = if cli.files.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.files.clone()
    };

    let mut rendered = Vec::new();
    let mut errors = Vec::new();
    for target in targets {
        match collect_target_entries(&target, &dir_options, &render_options) {
            Ok(entries) => {
                rendered.extend(entries);
            }
            Err(err) => {
                errors.push((target.clone(), err));
            }
        }
    }

    sort_entries(&mut rendered, render_options.sort);

    write_output(&rendered, &render_options, color_enabled, layout)?;

    if !errors.is_empty() {
        Err(anyhow::anyhow!(PartialFailure { errors }))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use fs::file::EntryKind;
    use fs::git::GitKind;
    use output::grid::render_grid_with_width;
    use output::render::RenderedEntry;

    fn parse_cli(args: &[&str]) -> Cli {
        let argv: Vec<&str> = std::iter::once("glas")
            .chain(args.iter().copied())
            .collect();
        Cli::parse_from(argv)
    }

    fn dummy_entry(path: &str) -> RenderedEntry {
        RenderedEntry {
            path: path.to_string(),
            kind: EntryKind::File,
            git: GitKind::Clean,
            mode: Some(0o100644),
            uid: Some(0),
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn tty_default_renderer_outputs_single_row() {
        let entries = vec![
            dummy_entry("Cargo.lock"),
            dummy_entry("Cargo.toml"),
            dummy_entry("README.md"),
            dummy_entry("README_ja.md"),
            dummy_entry("justfile"),
            dummy_entry("src"),
            dummy_entry("tests"),
        ];
        let mut out = String::new();
        render_grid_with_width(&entries, false, Some(9999), &mut out);

        assert_eq!(out.lines().count(), 1, "output was: {out:?}");
    }

    #[test]
    fn flatten_default_is_zero() {
        let cli = parse_cli(&[]);
        assert_eq!(cli.flatten, "0");
    }
}
