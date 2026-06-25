use rustc_hash::FxHashMap;

#[cfg(unix)]
use users::get_user_by_uid;

use crate::output::render::RenderedEntry;

/// Resolves the user name associated with the file entry's UID.
/// Searches the `user_cache` first; if not present, queries the system and caches the result.
/// Returns the numeric UID as a string if resolution fails.
pub fn long_user(entry: &RenderedEntry, user_cache: &mut FxHashMap<u32, String>) -> String {
    let Some(uid) = entry.uid else {
        return "-".to_string();
    };

    if let Some(user) = user_cache.get(&uid) {
        return user.clone();
    }

    #[cfg(unix)]
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());

    #[cfg(not(unix))]
    let user = uid.to_string();

    user_cache.insert(uid, user.clone());
    user
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::file::EntryKind;
    use crate::fs::git::GitKind;

    fn dummy_entry_with_uid(uid: Option<u32>) -> RenderedEntry {
        RenderedEntry {
            path: "test".to_string(),
            kind: EntryKind::File,
            git: GitKind::Clean,
            mode: Some(0o100644),
            uid,
            has_xattrs: false,
            size: 0,
            modified: None,
            stages: Vec::new(),
        }
    }

    #[test]
    fn test_long_user_none() {
        let entry = dummy_entry_with_uid(None);
        let mut cache = FxHashMap::default();
        assert_eq!(long_user(&entry, &mut cache), "-");
    }

    #[test]
    fn test_long_user_cached() {
        let entry = dummy_entry_with_uid(Some(1000));
        let mut cache = FxHashMap::default();
        cache.insert(1000, "cached_user".to_string());
        assert_eq!(long_user(&entry, &mut cache), "cached_user");
    }

    #[test]
    fn test_long_user_not_cached_missing() {
        let entry = dummy_entry_with_uid(Some(99999));
        let mut cache = FxHashMap::default();
        let result = long_user(&entry, &mut cache);
        // Verify cache consistency: whatever value was returned should be in the cache
        assert_eq!(cache.get(&99999).unwrap(), &result);
    }
}
