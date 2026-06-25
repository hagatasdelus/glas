use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

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
#[cfg(unix)]
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
#[cfg(unix)]
fn short_upper_h_is_header() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("a.txt"), "a\n").expect("write a");
    fs::set_permissions(dir.path().join("a.txt"), fs::Permissions::from_mode(0o644))
        .expect("set perms");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "-l", "-H", "--color=never"])
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

#[test]
fn test_treat_dirs_as_files_eza_behavior() {
    let repo = tempfile::TempDir::new().unwrap();
    let status = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    assert!(status.success());
    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "tester"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    fs::create_dir_all(repo.path().join("src/sub")).unwrap();
    fs::write(repo.path().join("src/sub/a.txt"), "hello").unwrap();
    std::process::Command::new("git")
        .args(["add", "src/sub/a.txt"])
        .current_dir(repo.path())
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(repo.path())
        .status()
        .unwrap();

    fs::write(repo.path().join("src/sub/a.txt"), "hello v2").unwrap();

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["-d", "src", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("src"), "should contain src: {text}");
    assert!(
        !text.contains("modified files"),
        "should not contain modified files summary: {text}"
    );
    assert!(
        !text.contains("a.txt"),
        "should not list directory contents: {text}"
    );
}

#[test]
fn test_only_dirs_and_only_files_coexist() {
    let dir = TempDir::new().expect("temp dir");
    fs::write(dir.path().join("a.txt"), "hello").expect("write a");
    fs::create_dir(dir.path().join("subdir")).expect("create subdir");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(dir.path())
        .args(["--no-git", "-D", "-f", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("a.txt"), "should contain file: {text}");
    assert!(text.contains("subdir"), "should contain directory: {text}");
}
