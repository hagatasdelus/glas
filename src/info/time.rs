//! # info/time
//!
//! ファイルエントリーの最終更新日時を、指定されたタイムゾーンオフセットに従って
//! 指定フォーマットの文字列に変換する処理を提供するモジュールです。

use std::time::SystemTime;
use time::{OffsetDateTime, UtcOffset, macros::format_description};

/// 最終更新日時を `[day] [month] [hour]:[minute]` 形式の文字列にフォーマットします。
/// 日時情報がない場合は `"-"` を返します。
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
