use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use time::OffsetDateTime;

fn main() -> io::Result<()> {
    let ver_base = version_string();
    let time_str = build_time();

    let ver = is_development_version()
        .then(git_hash)
        .flatten()
        .map(|git_meta| format!("{} {} ({})", ver_base, git_meta, time_str))
        .unwrap_or_else(|| format!("{} ({})", ver_base, time_str));

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let path = out.join("version_string.txt");

    let mut f = File::create(&path)
        .unwrap_or_else(|_| panic!("Failed to create {}", path.to_string_lossy()));

    write!(f, "{}", ver)?;

    Ok(())
}

fn git_hash() -> Option<String> {
    Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .inspect(|_| {
            println!("cargo:rerun-if-changed=.git/HEAD");
            println!("cargo:rerun-if-changed=.git/refs/tags");
            println!("cargo:rerun-if-changed=.git/packed-refs");
        })
        .map(|output| {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            version.strip_prefix('v').unwrap_or(&version).to_string()
        })
}

fn is_debug_build() -> bool {
    env::var("OPT_LEVEL").is_ok_and(|v| v == "0")
        || env::var("DEBUG").is_ok_and(|v| v != "0" && v != "false")
}

fn is_development_version() -> bool {
    cargo_version().ends_with("-pre") || is_debug_build()
}

fn cargo_version() -> String {
    env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set")
}

fn version_string() -> String {
    let ver = cargo_version();
    let feats = nonstandard_features_string();

    if feats.is_empty() {
        ver
    } else {
        format!("{} [{}]", ver, feats)
    }
}

fn feature_enabled(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name.to_uppercase())).is_ok_and(|e| !e.is_empty())
}

fn nonstandard_features_string() -> String {
    if feature_enabled("git") {
        "+git"
    } else {
        "-git"
    }
    .to_string()
}

fn build_time() -> String {
    let now = OffsetDateTime::now_utc();
    format!(
        "{}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        u8::from(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}
