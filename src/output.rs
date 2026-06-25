//! # output
//!
//! Handles rendering logic for different layout formats (long table, grid, custom format, etc.)
//! and sorts collected file entries before writing to stdout.

pub mod custom;
pub mod grid;
pub mod render;
pub mod table;

pub use render::sort_entries;
pub use table::write_output;
