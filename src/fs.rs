//! # fs
//!
//! Provides directory traversal, file metadata collection, and Git repository status querying capabilities.

pub mod dir;
pub mod file;
pub mod git;

pub use dir::collect_target_entries;
