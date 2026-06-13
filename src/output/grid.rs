use terminal_size::{Width, terminal_size};

use crate::output::render::RenderedEntry;
use crate::theme::apply_color;

fn get_terminal_width() -> Option<usize> {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse::<usize>().ok())
        .or_else(|| terminal_size().map(|(Width(w), _)| w as usize))
}

pub fn render_grid(entries: &[RenderedEntry], color_enabled: bool, out: &mut String) {
    if entries.is_empty() {
        return;
    }

    let Some(term_width) = get_terminal_width() else {
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
                let rendered = apply_color(&entry.path, entry, color_enabled, false);
                if c == num_cols - 1 || idx + num_rows >= entries.len() {
                    out.push_str(&rendered);
                } else {
                    let padding = col_width.saturating_sub(entry.path.len());
                    out.push_str(&rendered);
                    out.push_str(&" ".repeat(padding));
                }
            }
        }
        out.push('\n');
    }
}
