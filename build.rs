use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use time::OffsetDateTime;

fn main() -> io::Result<()> {
    let tagline = "glas - A Git-aware ls alternative";
    let url = "https://github.com/hagatasdelus/glas";

    let ver = if is_debug_build() {
        format!(
            "{}\nv{} (pre-release debug build!)\n{}",
            tagline,
            version_string(),
            url
        )
    } else if is_development_version() {
        format!(
            "{}\nv{} [{}] built on {} (pre-release!)\n{}",
            tagline,
            version_string(),
            git_hash(),
            build_date(),
            url
        )
    } else {
        format!("{}\nv{}\n{}", tagline, version_string(), url)
    };

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let path = out.join("version_string.txt");

    let mut f = File::create(&path)
        .unwrap_or_else(|_| panic!("Failed to create {}", path.to_string_lossy()));
    writeln!(f, "{}", ver)?;

    Ok(())
}

fn git_hash() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| {
            let hash = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if hash.is_empty() {
                "unknown".to_string()
            } else {
                hash
            }
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn is_development_version() -> bool {
    cargo_version().ends_with("-pre") || env::var("PROFILE").unwrap_or_default() == "debug"
}

fn is_debug_build() -> bool {
    env::var("PROFILE").unwrap_or_default() == "debug"
}

fn cargo_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap()
}

fn version_string() -> String {
    let mut ver = cargo_version();
    let feats = nonstandard_features_string();
    if !feats.is_empty() {
        ver.push_str(&format!(" [{}]", &feats));
    }
    ver
}

fn feature_enabled(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name.to_uppercase()))
        .map(|e| !e.is_empty())
        .unwrap_or(false)
}

fn nonstandard_features_string() -> String {
    let mut s = Vec::new();
    if feature_enabled("git") {
        s.push("+git");
    } else {
        s.push("-git");
    }
    s.join(", ")
}

fn build_date() -> String {
    let now = OffsetDateTime::now_utc();
    format!(
        "{}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    )
}
