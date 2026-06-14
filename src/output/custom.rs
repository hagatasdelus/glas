//! # output/custom
//!
//! Provides rendering logic for file entries based on user-defined custom format templates.

use std::time::UNIX_EPOCH;

use crate::fs::file::EntryKind;
use crate::output::render::RenderedEntry;

/// Generates a rendered string for a file entry by substituting placeholders in the specified custom template
/// (e.g., `%(path) %(size)`).
pub fn render_custom_format(template: &str, entry: &RenderedEntry) -> String {
    let kind = match entry.kind {
        EntryKind::File => "file",
        EntryKind::Directory => "dir",
        EntryKind::Summary { .. } => "summary",
    };

    let modified = entry
        .modified
        .and_then(|mtime| mtime.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|| "-".to_string());

    let mut rendered = template.to_string();
    rendered = rendered.replace("%(path)", &entry.path);
    rendered = rendered.replace("%(size)", &entry.size.to_string());
    rendered = rendered.replace("%(modified)", &modified);
    rendered = rendered.replace("%(git)", entry.git.tag());
    rendered = rendered.replace("%(type)", kind);
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::git::GitKind;
    use std::time::SystemTime;

    #[test]
    fn test_render_custom_format_file() {
        let entry = RenderedEntry {
            path: "test_file.txt".to_string(),
            kind: EntryKind::File,
            git: GitKind::Modified,
            mode: Some(0o100644),
            uid: Some(1000),
            has_xattrs: false,
            size: 1234,
            modified: Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(100000)),
            stages: Vec::new(),
        };

        let template =
            "Path: %(path), Size: %(size), Modified: %(modified), Git: %(git), Type: %(type)";
        let result = render_custom_format(template, &entry);
        assert_eq!(
            result,
            "Path: test_file.txt, Size: 1234, Modified: 100000, Git: M, Type: file"
        );
    }

    #[test]
    fn test_render_custom_format_directory_no_modified() {
        let entry = RenderedEntry {
            path: "test_dir".to_string(),
            kind: EntryKind::Directory,
            git: GitKind::Clean,
            mode: Some(0o40755),
            uid: Some(1000),
            has_xattrs: false,
            size: 4096,
            modified: None,
            stages: Vec::new(),
        };

        let template = "%(type) %(path) %(size) %(modified) %(git)";
        let result = render_custom_format(template, &entry);
        assert_eq!(result, "dir test_dir 4096 -  ");
    }

    #[test]
    fn test_render_custom_format_summary() {
        let entry = RenderedEntry {
            path: "summary_path".to_string(),
            kind: EntryKind::Summary { modified_count: 5 },
            git: GitKind::Conflicted,
            mode: None,
            uid: None,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        };

        let template = "%(type) %(path) %(git)";
        let result = render_custom_format(template, &entry);
        assert_eq!(result, "summary summary_path !");
    }
}
