use assert_cmd::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

#[test]
fn null_output_is_nul_separated() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");
    fs::write(dir.path().join("file with space.txt"), "b\n").expect("write b");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "-z", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    assert!(output.contains(&0), "stdout was: {:?}", output);
    assert!(output.windows("a.txt\0".len()).any(|w| w == b"a.txt\0"));
    assert!(
        output
            .windows("file with space.txt\0".len())
            .any(|w| w == b"file with space.txt\0")
    );
}

#[test]
fn long_format_marks_xattr_with_at_sign() {
    let dir = TempDir::new().expect("temp dir");
    let file_path = dir.path().join("a.txt");
    fs::write(&file_path, "a\n").expect("write a");
    fs::set_permissions(&file_path, fs::Permissions::from_mode(0o644)).expect("set perms");

    // Skip test if xattrs are not supported on this filesystem
    if xattr::set(&file_path, "com.glas.test", b"1").is_err() {
        eprintln!("Skipping test: xattrs not supported on this filesystem");
        return;
    }

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "-l", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    let first_line = text.lines().next().unwrap_or("");
    assert!(first_line.contains(".rw-r--r--@"), "stdout was: {text}");
}

#[test]
fn short_h_is_header_not_help() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");
    fs::set_permissions(dir.path().join("a.txt"), fs::Permissions::from_mode(0o644))
        .expect("set perms");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "-l", "-h", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    let first_line = text.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("GIT Permissions Size User"),
        "stdout was: {text}"
    );
    assert!(
        first_line.contains("Date Modified Name"),
        "stdout was: {text}"
    );

    let second_line = text.lines().nth(1).unwrap_or("");
    assert!(second_line.contains(".rw-r--r--"), "stdout was: {text}");
}

#[test]
fn test_multiple_targets_partial_failure() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "a.txt", "non_existent_file", "--color=never"])
        .assert()
        .failure()
        .code(1)
        .get_output()
        .clone();

    let stdout_text = String::from_utf8(output.stdout).expect("utf8 stdout");
    let stderr_text = String::from_utf8(output.stderr).expect("utf8 stderr");

    assert!(stdout_text.contains("a.txt"), "stdout was: {stdout_text}");
    assert!(
        stderr_text.contains("non_existent_file"),
        "stderr was: {stderr_text}"
    );
}
