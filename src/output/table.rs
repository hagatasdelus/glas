//! # output/table
//!
//! Provides functionality for writing file entries to standard output.
//! Supports grid view, single-column view, detailed table view, and custom format view.

use anyhow::{Context, Result};
use rustc_hash::FxHashMap;
use std::io::{self, Write};
use time::UtcOffset;

use crate::info::{long_modified, long_size, long_user, permission_string};
use crate::options::RenderOptions;
use crate::options::layout::OutputLayout;
use crate::output::custom::render_custom_format;
use crate::output::grid::render_grid;
use crate::output::render::RenderedEntry;
use crate::theme::apply_color;

#[derive(Debug)]
struct LongDisplayRow {
    permissions: String,
    size: String,
    user: String,
    modified: String,
    name: String,
    git: String,
}

#[derive(Default)]
struct TableWidths {
    pub git: usize,
    pub permissions: usize,
    pub size: usize,
    pub user: usize,
    pub modified: usize,
}

impl TableWidths {
    pub fn calculate(rows: &[LongDisplayRow]) -> Self {
        let mut widths = TableWidths {
            git: "GIT".len(),
            permissions: "Permissions".len(),
            size: "Size".len(),
            user: "User".len(),
            modified: "Date Modified".len(),
        };

        for row in rows {
            widths.git = widths.git.max(row.git.len());
            widths.permissions = widths.permissions.max(row.permissions.len());
            widths.size = widths.size.max(row.size.len());
            widths.user = widths.user.max(row.user.len());
            widths.modified = widths.modified.max(row.modified.len());
        }
        widths
    }
}

struct TableRenderer<'a> {
    widths: TableWidths,
    options: &'a RenderOptions,
    color_enabled: bool,
}

impl<'a> TableRenderer<'a> {
    pub fn new(options: &'a RenderOptions, widths: TableWidths, color_enabled: bool) -> Self {
        Self {
            widths,
            options,
            color_enabled,
        }
    }

    pub fn render_header(&self) -> String {
        format!(
            "{:<git_width$} {:<permissions_width$} {:>size_width$} {:<user_width$} {:<modified_width$} {}",
            "GIT",
            "Permissions",
            "Size",
            "User",
            "Date Modified",
            "Name",
            git_width = self.widths.git,
            permissions_width = self.widths.permissions,
            size_width = self.widths.size,
            user_width = self.widths.user,
            modified_width = self.widths.modified
        )
    }

    pub fn render_row(&self, entry: &RenderedEntry, row: &LongDisplayRow) -> String {
        let rendered = format!(
            "{:<git_width$} {:<permissions_width$} {:>size_width$} {:<user_width$} {:<modified_width$} {}",
            entry.git.tag(),
            row.permissions,
            row.size,
            row.user,
            row.modified,
            row.name,
            git_width = self.widths.git,
            permissions_width = self.widths.permissions,
            size_width = self.widths.size,
            user_width = self.widths.user,
            modified_width = self.widths.modified
        );
        apply_color(&rendered, entry, self.color_enabled, self.options.long).into_owned()
    }
}

/// Formats and writes the sorted list of `RenderedEntry` to standard output (stdout)
/// according to the specified output layout mode (grid, long table, single line, custom format, etc.).
pub fn write_output(
    entries: &[RenderedEntry],
    options: &RenderOptions,
    color_enabled: bool,
    layout: OutputLayout,
) -> Result<()> {
    let separator = if options.null { "\0" } else { "\n" };
    let mut out = String::new();

    if options.stage {
        for entry in entries {
            if entry.stages.is_empty() {
                let rendered = apply_color(&entry.path, entry, color_enabled, options.long);
                out.push_str(&rendered);
                out.push_str(separator);
            } else {
                for stage_info in &entry.stages {
                    let path = apply_color(&entry.path, entry, color_enabled, options.long);
                    let line = format!(
                        "{:06o} {} {}\t{}",
                        stage_info.mode, stage_info.object_id, stage_info.stage, path
                    );
                    out.push_str(&line);
                    out.push_str(separator);
                }
            }
        }
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(out.as_bytes())
            .context("failed to write output")?;
        return Ok(());
    }

    if matches!(layout, OutputLayout::Long) {
        let mut user_cache: FxHashMap<u32, String> = FxHashMap::default();
        let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        let rows: Vec<LongDisplayRow> = entries
            .iter()
            .map(|entry| LongDisplayRow {
                permissions: permission_string(entry),
                size: long_size(entry),
                user: long_user(entry, &mut user_cache),
                modified: long_modified(entry.modified, offset),
                name: entry.path.clone(),
                git: entry.git.tag().to_string(),
            })
            .collect();

        let widths = TableWidths::calculate(&rows);
        let renderer = TableRenderer::new(options, widths, color_enabled);

        if options.header {
            out.push_str(&renderer.render_header());
            out.push_str(separator);
        }

        for (entry, row) in entries.iter().zip(rows.iter()) {
            out.push_str(&renderer.render_row(entry, row));
            out.push_str(separator);
        }

        let mut stdout = io::stdout().lock();
        stdout
            .write_all(out.as_bytes())
            .context("failed to write output")?;
        return Ok(());
    }

    if matches!(layout, OutputLayout::Grid) {
        render_grid(entries, color_enabled, &mut out);
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(out.as_bytes())
            .context("failed to write output")?;
        return Ok(());
    }

    for entry in entries {
        let rendered = if matches!(layout, OutputLayout::Custom) {
            let template = options.format.as_deref().unwrap_or("%(path)");
            render_custom_format(template, entry)
        } else {
            entry.path.clone()
        };

        out.push_str(&apply_color(&rendered, entry, color_enabled, options.long));
        out.push_str(separator);
    }

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(out.as_bytes())
        .context("failed to write output")?;
    Ok(())
}
