use std::time::UNIX_EPOCH;

use crate::fs::file::EntryKind;
use crate::output::render::RenderedEntry;

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
