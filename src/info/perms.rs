//! # info/perms
//!
//! Provides formatting logic to display file permission (Unix permission) information
//! in a human-readable string (e.g., `.rw-r--r--@`).

use crate::fs::file::EntryKind;
use crate::output::render::RenderedEntry;

/// Generates an 11-character permission string integrating ownership type,
/// permission mode bits, and extended attributes.
pub fn permission_string(entry: &RenderedEntry) -> String {
    if matches!(entry.kind, EntryKind::Summary { .. }) {
        return "d--------- ".to_string();
    }

    let Some(mode) = entry.mode else {
        return "---------- ".to_string();
    };

    let mut buf = String::with_capacity(11);
    buf.push(file_type_char(mode));
    buf.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    buf.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    buf.push(match (mode & 0o100 != 0, mode & 0o4000 != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    });
    buf.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    buf.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    buf.push(match (mode & 0o010 != 0, mode & 0o2000 != 0) {
        (true, true) => 's',
        (false, true) => 'S',
        (true, false) => 'x',
        (false, false) => '-',
    });
    buf.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    buf.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    buf.push(match (mode & 0o001 != 0, mode & 0o1000 != 0) {
        (true, true) => 't',
        (false, true) => 'T',
        (true, false) => 'x',
        (false, false) => '-',
    });
    buf.push(if entry.has_xattrs { '@' } else { ' ' });
    buf
}

fn file_type_char(mode: u32) -> char {
    match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        0o010000 => 'p',
        0o020000 => 'c',
        0o060000 => 'b',
        0o140000 => 's',
        _ => '.',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use crate::fs::git::GitKind;

    fn dummy_entry_with_mode(mode: u32) -> RenderedEntry {
        RenderedEntry {
            path: "test".to_string(),
            kind: EntryKind::File,
            git: GitKind::Clean,
            mode: Some(mode),
            uid: Some(0),
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn test_permission_string() {
        let entry = dummy_entry_with_mode(0o100644);
        assert_eq!(permission_string(&entry), ".rw-r--r-- ");

        let mut entry = dummy_entry_with_mode(0o040755);
        entry.kind = EntryKind::Directory;
        assert_eq!(permission_string(&entry), "drwxr-xr-x ");

        let entry = dummy_entry_with_mode(0o104755);
        assert_eq!(permission_string(&entry), ".rwsr-xr-x ");

        let entry = dummy_entry_with_mode(0o102755);
        assert_eq!(permission_string(&entry), ".rwxr-sr-x ");

        let entry = dummy_entry_with_mode(0o101755);
        assert_eq!(permission_string(&entry), ".rwxr-xr-t ");

        let mut entry = dummy_entry_with_mode(0o100644);
        entry.has_xattrs = true;
        assert_eq!(permission_string(&entry), ".rw-r--r--@");

        let mut entry_none = dummy_entry_with_mode(0);
        entry_none.mode = None;
        assert_eq!(permission_string(&entry_none), "---------- ");

        let mut entry_summary = dummy_entry_with_mode(0o100644);
        entry_summary.kind = EntryKind::Summary { modified_count: 1 };
        assert_eq!(permission_string(&entry_summary), "d--------- ");

        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o120777)),
            "lrwxrwxrwx "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o010666)),
            "prw-rw-rw- "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o020600)),
            "crw------- "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o060640)),
            "brw-r----- "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o140755)),
            "srwxr-xr-x "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o000755)),
            ".rwxr-xr-x "
        );

        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o104644)),
            ".rwSr--r-- "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o102644)),
            ".rw-r-Sr-- "
        );
        assert_eq!(
            permission_string(&dummy_entry_with_mode(0o101644)),
            ".rw-r--r-T "
        );
    }
}
