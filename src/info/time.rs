//! # info/time
//!
//! Provides formatting logic to convert the last modification time of a file entry
//! to a string using a specified timezone offset and format.

use std::time::SystemTime;
use time::{OffsetDateTime, UtcOffset, macros::format_description};

/// Formats the last modification time into a string with the `[day] [month] [hour]:[minute]` format.
/// Returns `"-"` if no time information is available.
pub fn long_modified(modified: Option<SystemTime>, offset: UtcOffset) -> String {
    const DATE_FORMAT: &[time::format_description::FormatItem<'static>] =
        format_description!("[day padding:zero] [month repr:short] [hour]:[minute]");

    let Some(modified) = modified else {
        return "-".to_string();
    };
    let date = OffsetDateTime::from(modified).to_offset(offset);
    date.format(DATE_FORMAT).unwrap_or_else(|_| "-".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use time::UtcOffset;

    #[test]
    fn test_long_modified_none() {
        assert_eq!(long_modified(None, UtcOffset::UTC), "-");
    }

    #[test]
    fn test_long_modified_some() {
        let t = SystemTime::UNIX_EPOCH;
        assert_eq!(long_modified(Some(t), UtcOffset::UTC), "01 Jan 00:00");
    }
}
