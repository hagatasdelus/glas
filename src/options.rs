//! # options
//!
//! This module provides configuration options, including command-line argument parsing (`cli`),
//! filtering/display configuration (`config`), and output layout determination (`layout`).

pub mod cli;
pub mod config;
pub mod layout;

pub use cli::{Cli, ColorWhen, SortField};
pub use config::{DirOptions, RenderOptions};
pub use layout::resolve_layout_mode;
