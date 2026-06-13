//! # options
//!
//! このモジュールは、コマンドライン引数のパース（`cli`）、表示やフィルタリングのオプション設定（`config`）、
//! 出力レイアウトの決定（`layout`）などの設定オプション全般を提供します。

pub mod cli;
pub mod config;
pub mod layout;

pub use cli::{Cli, ColorWhen, SortField};
pub use config::{DirOptions, RenderOptions};
pub use layout::resolve_layout_mode;
