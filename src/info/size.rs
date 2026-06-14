//! # info/size
//!
//! Provides size formatting logic to convert file sizes in bytes into a human-readable format
//! (e.g., `1k`, `1.5M`).

use crate::fs::file::EntryKind;
use crate::output::render::RenderedEntry;

/// Generates a file size string used for detailed table display.
/// Returns `"-"` if the entry is not a regular file (e.g., directories).
pub fn long_size(entry: &RenderedEntry) -> String {
    if !matches!(entry.kind, EntryKind::File) {
        return "-".to_string();
    }
    human_size(entry.size)
}

/// Converts a numeric size in bytes to a string with a human-readable unit (k, M, G, T...).
pub fn human_size(size: u64) -> String {
    if size < 1024 {
        return size.to_string();
    }

    const UNITS: [&str; 6] = ["k", "M", "G", "T", "P", "E"];
    let mut value = size as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        if value >= 1024.0 {
            unit_index += 1;
        }
    }

    if value >= 10.0 || value.fract() < 0.05 {
        format!("{value:.0}{}", UNITS[unit_index])
    } else {
        format!("{value:.1}{}", UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size() {
        assert_eq!(human_size(0), "0");
        assert_eq!(human_size(500), "500");
        assert_eq!(human_size(1023), "1023");
        assert_eq!(human_size(1024), "1k");
        assert_eq!(human_size(1500), "1.5k");
        assert_eq!(human_size(10240), "10k");
        assert_eq!(human_size(1048576), "1M");
    }
}
