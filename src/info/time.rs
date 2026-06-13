use std::time::SystemTime;
use time::{OffsetDateTime, UtcOffset, macros::format_description};

pub fn long_modified(modified: Option<SystemTime>, offset: UtcOffset) -> String {
    const DATE_FORMAT: &[time::format_description::FormatItem<'static>] =
        format_description!("[day padding:zero] [month repr:short] [hour]:[minute]");

    let Some(modified) = modified else {
        return "-".to_string();
    };
    let date = OffsetDateTime::from(modified).to_offset(offset);
    date.format(DATE_FORMAT).unwrap_or_else(|_| "-".to_string())
}
