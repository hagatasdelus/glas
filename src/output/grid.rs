//! # output/grid
//!
//! Provides grid layout processing to arrange and display file entries in a grid (multiple columns)
//! aligned to the terminal width.

use terminal_size::{Width, terminal_size};

use crate::output::render::RenderedEntry;
use crate::theme::apply_color;

fn get_terminal_width() -> Option<usize> {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse::<usize>().ok())
        .or_else(|| terminal_size().map(|(Width(w), _)| w as usize))
}

/// Arranges file entries in a grid layout based on the terminal width and writes the result to the output buffer.
/// If the terminal width cannot be determined, it falls back to displaying all entries in a single row separated by spaces.
pub fn render_grid(entries: &[RenderedEntry], color_enabled: bool, out: &mut String) {
    render_grid_with_width(entries, color_enabled, get_terminal_width(), out);
}

/// Internal helper that arranges file entries in a grid layout with an explicit terminal width parameter.
/// If `term_width` is None, it falls back to displaying all entries in a single row separated by spaces.
pub(crate) fn render_grid_with_width(
    entries: &[RenderedEntry],
    color_enabled: bool,
    term_width: Option<usize>,
    out: &mut String,
) {
    if entries.is_empty() {
        return;
    }

    let Some(term_width) = term_width else {
        // Fall back to printing everything in a single row (legacy/test behavior when terminal size is unavailable)
        for (idx, entry) in entries.iter().enumerate() {
            if idx > 0 {
                out.push_str("  ");
            }
            let rendered = apply_color(&entry.path, entry, color_enabled, false);
            out.push_str(&rendered);
        }
        out.push('\n');
        return;
    };

    // Find the max length of the uncolored path
    let max_len = entries.iter().map(|e| e.path.len()).max().unwrap_or(0);
    let col_width = max_len + 2; // path width + spacing

    let num_cols = (term_width / col_width).max(1);
    let num_rows = entries.len().div_ceil(num_cols);

    // Fill the grid row by row (vertical-first ordering, like ls/eza)
    for r in 0..num_rows {
        for c in 0..num_cols {
            let idx = c * num_rows + r;
            if idx < entries.len() {
                let entry = &entries[idx];
                let is_last_col = c == num_cols - 1 || idx + num_rows >= entries.len();
                render_grid_cell(entry, color_enabled, col_width, is_last_col, out);
            }
        }
        out.push('\n');
    }
}

fn render_grid_cell(
    entry: &RenderedEntry,
    color_enabled: bool,
    col_width: usize,
    is_last_col: bool,
    out: &mut String,
) {
    let rendered = apply_color(&entry.path, entry, color_enabled, false);
    if is_last_col {
        out.push_str(&rendered);
    } else {
        let padding = col_width.saturating_sub(entry.path.len());
        out.push_str(&rendered);
        out.push_str(&" ".repeat(padding));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use crate::fs::git::GitKind;

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
    fn test_render_grid_empty() {
        let mut out = String::new();
        render_grid(&[], false, &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn test_render_grid_fallback_no_term_width() {
        let entries = vec![dummy_entry("a"), dummy_entry("bb"), dummy_entry("ccc")];
        let mut out = String::new();
        render_grid_with_width(&entries, false, None, &mut out);

        assert_eq!(out, "a  bb  ccc\n");
    }

    #[test]
    fn test_render_grid_with_width() {
        let entries = vec![
            dummy_entry("a"),
            dummy_entry("bb"),
            dummy_entry("ccc"),
            dummy_entry("d"),
            dummy_entry("e"),
        ];
        let mut out = String::new();
        render_grid_with_width(&entries, false, Some(20), &mut out);

        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "a    ccc  e");
        assert_eq!(lines[1], "bb   d");
    }
}
