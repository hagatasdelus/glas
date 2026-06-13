use owo_colors::OwoColorize;

use crate::fs::git::GitKind;
use crate::output::render::RenderedEntry;

pub fn apply_color(
    rendered: &str,
    entry: &RenderedEntry,
    color_enabled: bool,
    _long: bool,
) -> String {
    if !color_enabled {
        return rendered.to_string();
    }

    match entry.git {
        GitKind::Conflicted => rendered.red().to_string(),
        GitKind::Staged => rendered.green().to_string(),
        GitKind::Modified => rendered.yellow().to_string(),
        GitKind::Deleted => rendered.red().to_string(),
        GitKind::Untracked => rendered.cyan().to_string(),
        GitKind::Ignored => rendered.bright_black().to_string(),
        GitKind::Clean => rendered.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use owo_colors::OwoColorize;

    fn dummy_entry_with_git(git: GitKind) -> RenderedEntry {
        RenderedEntry {
            path: "test".to_string(),
            kind: EntryKind::File,
            git,
            mode: Some(0o100644),
            uid: Some(0),
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn test_apply_color_disabled() {
        let entry = dummy_entry_with_git(GitKind::Modified);
        assert_eq!(apply_color("hello", &entry, false, false), "hello");
    }

    #[test]
    fn test_apply_color_git_kinds() {
        let entry = dummy_entry_with_git(GitKind::Conflicted);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".red().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Staged);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".green().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Deleted);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".red().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Modified);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".yellow().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Untracked);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".cyan().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Ignored);
        assert_eq!(
            apply_color("hello", &entry, true, false),
            "hello".bright_black().to_string()
        );

        let entry = dummy_entry_with_git(GitKind::Clean);
        assert_eq!(apply_color("hello", &entry, true, false), "hello");
    }
}
