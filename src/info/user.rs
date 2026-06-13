use rustc_hash::FxHashMap;
use users::get_user_by_uid;

use crate::output::render::RenderedEntry;

pub fn long_user(entry: &RenderedEntry, user_cache: &mut FxHashMap<u32, String>) -> String {
    let Some(uid) = entry.uid else {
        return "-".to_string();
    };

    if let Some(user) = user_cache.get(&uid) {
        return user.clone();
    }

    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());
    user_cache.insert(uid, user.clone());
    user
}
