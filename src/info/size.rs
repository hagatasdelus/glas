//! # info/size
//!
//! ファイルサイズを人間が読みやすい形式（例: `1k`, `1.5M`）に変換するフォーマット処理を
//! 提供するモジュールです。

use crate::fs::file::EntryKind;
use crate::output::render::RenderedEntry;

/// テーブル詳細表示で用いるファイルサイズの文字列を生成します。
/// ファイルでないエントリー（ディレクトリなど）の場合は `"-"` を返します。
pub fn long_size(entry: &RenderedEntry) -> String {
    if !matches!(entry.kind, EntryKind::File) {
        return "-".to_string();
    }
    human_size(entry.size)
}

/// バイト単位の数値を人間が読みやすい単位（k, M, G, T...）に丸めた文字列に変換します。
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
