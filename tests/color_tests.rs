use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn default_colors_match_long_for_clean_directories() {
    let dir = TempDir::new().expect("temp dir");
    fs::create_dir_all(dir.path().join("clean_dir")).expect("create clean dir");

    let mut default_cmd = Command::cargo_bin("glas").expect("binary");
    let default_output = default_cmd
        .current_dir(dir.path())
        .args(["--no-git", "--color=always"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let mut long_cmd = Command::cargo_bin("glas").expect("binary");
    let long_output = long_cmd
        .current_dir(dir.path())
        .args(["--no-git", "-l", "--color=always"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(default_output).expect("utf8 output");
    let long_text = String::from_utf8(long_output).expect("utf8 output");
    assert!(
        text.contains("\u{1b}[34mclean_dir\u{1b}[39m"),
        "stdout should color clean directory blue: {text:?}"
    );
    assert!(
        long_text.contains("\u{1b}[34m"),
        "long output should use same clean-directory color policy: {long_text:?}"
    );
}

#[test]
fn special_files_are_bold_in_default_output() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("Cargo.toml"), "[package]\nname='x'\n").expect("write cargo");
    fs::write(dir.path().join("justfile"), "default:\n\t@echo ok\n").expect("write justfile");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "--color=always"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).expect("utf8 output");

    assert!(
        text.contains("\u{1b}[1mCargo.toml\u{1b}[0m"),
        "Cargo.toml should be bold: {text:?}"
    );
    assert!(
        text.contains("\u{1b}[1mjustfile\u{1b}[0m"),
        "justfile should be bold: {text:?}"
    );
}

#[test]
fn special_files_are_bold_even_when_modified_or_readme() {
    let temp = TempDir::new().expect("failed to create temp dir");
    let status = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init");
    assert!(status.success());
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "tester"])
        .current_dir(temp.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp.path())
        .status()
        .unwrap();

    let cargo_path = temp.path().join("Cargo.toml");
    let readme_path = temp.path().join("README.md");
    let readme_ja_path = temp.path().join("README_ja.md");
    fs::write(&cargo_path, "[package]\nname='x'\n").unwrap();
    fs::write(&readme_path, "# README\n").unwrap();
    fs::write(&readme_ja_path, "# README JA\n").unwrap();

    std::process::Command::new("git")
        .args(["add", "Cargo.toml", "README.md", "README_ja.md"])
        .current_dir(temp.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(temp.path())
        .status()
        .unwrap();

    fs::write(&cargo_path, "[package]\nname='y'\n").unwrap();
    fs::write(&readme_path, "# README v2\n").unwrap();

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(temp.path())
        .args(["--color=always"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).expect("utf8 output");

    assert!(text.contains("Cargo.toml"), "stdout was: {text:?}");
    assert!(
        text.contains("\u{1b}[1m"),
        "should contain bold escape: {text:?}"
    );
    assert!(
        text.contains("\u{1b}[33m"),
        "should contain yellow escape: {text:?}"
    );

    assert!(text.contains("README.md"), "stdout was: {text:?}");
    assert!(text.contains("README_ja.md"), "stdout was: {text:?}");
}
