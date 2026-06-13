use assert_cmd::Command;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn git(path: &Path, args: &[&str]) {
    let status = StdCommand::new("git")
        .args(args)
        .current_dir(path)
        .status()
        .expect("failed to execute git");
    assert!(status.success(), "git command failed: {:?}", args);
}

fn init_repo() -> TempDir {
    let temp = TempDir::new().expect("failed to create temp dir");
    git(temp.path(), &["init", "-q"]);
    git(temp.path(), &["config", "user.email", "test@example.com"]);
    git(temp.path(), &["config", "user.name", "tester"]);
    git(temp.path(), &["config", "commit.gpgsign", "false"]);
    git(temp.path(), &["config", "tag.gpgsign", "false"]);
    temp
}

#[test]
fn git_only_filters_out_untracked_files() {
    let repo = init_repo();
    fs::write(repo.path().join("tracked.txt"), "v1\n").expect("write tracked");
    git(repo.path(), &["add", "tracked.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "init"]);

    fs::write(repo.path().join("tracked.txt"), "v2\n").expect("update tracked");
    fs::write(repo.path().join("new.txt"), "new\n").expect("write untracked");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--cached", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    assert!(text.contains("tracked.txt"), "stdout was: {text}");
    assert!(!text.contains("new.txt"), "stdout was: {text}");
}

#[test]
fn full_name_prints_repo_relative_paths() {
    let repo = init_repo();
    fs::create_dir_all(repo.path().join("sub")).expect("create subdir");
    fs::write(repo.path().join("sub/file.txt"), "v1\n").expect("write file");
    git(repo.path(), &["add", "sub/file.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "init"]);
    fs::write(repo.path().join("sub/file.txt"), "v2\n").expect("modify file");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path().join("sub"))
        .args(["--cached", "--full-name", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    assert!(text.contains("sub/file.txt"), "stdout was: {text}");
}

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
fn short_help_uses_question_mark() {
    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd.arg("-?").assert().success().get_output().stdout.clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(text.contains("USAGE:"), "stdout was: {text}");
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
fn show_ignored_includes_ignored_files() {
    let repo = init_repo();
    fs::write(repo.path().join(".gitignore"), "ignored.log\n").expect("write gitignore");
    git(repo.path(), &["add", ".gitignore"]);
    git(repo.path(), &["commit", "-q", "-m", "add gitignore"]);
    fs::write(repo.path().join("ignored.log"), "content\n").expect("write ignored");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--show-ignored", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    assert!(text.contains("ignored.log"), "stdout was: {text}");
}

#[test]
fn long_format_marks_xattr_with_at_sign() {
    let dir = TempDir::new().expect("temp dir");
    let file_path = dir.path().join("a.txt");
    fs::write(&file_path, "a\n").expect("write a");
    fs::set_permissions(&file_path, fs::Permissions::from_mode(0o644)).expect("set perms");
    xattr::set(&file_path, "com.glas.test", b"1").expect("set xattr");

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
fn long_format_git_column_is_left_aligned() {
    let repo = init_repo();
    fs::write(repo.path().join("tracked.txt"), "v1\n").expect("write tracked");
    git(repo.path(), &["add", "tracked.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "init"]);
    fs::write(repo.path().join("tracked.txt"), "v2\n").expect("update tracked");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--modified", "-l", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 output");
    let first_line = text.lines().next().unwrap_or("");
    assert!(first_line.starts_with("M"), "stdout was: {text}");
}

#[test]
fn help_mentions_flatten_default_zero() {
    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd.arg("-?").assert().success().get_output().stdout.clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(
        text.contains("flatten nested modified files [default: 0]"),
        "stdout was: {text}"
    );
}

#[test]
fn short_help_uses_grouped_sections_without_arguments_block() {
    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd.arg("-?").assert().success().get_output().stdout.clone();
    let text = String::from_utf8(output).expect("utf8 output");

    assert!(!text.contains("Arguments:"), "stdout was: {text}");
    assert!(text.contains("META OPTIONS"), "stdout was: {text}");
    assert!(text.contains("DISPLAY OPTIONS"), "stdout was: {text}");
    assert!(
        text.contains("FILTERING AND SORTING OPTIONS"),
        "stdout was: {text}"
    );
    assert!(text.contains("GIT-AWARE OPTIONS"), "stdout was: {text}");
}

#[test]
fn short_help_mentions_only_short_flag() {
    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd.arg("-?").assert().success().get_output().stdout.clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(text.contains("-?"), "stdout was: {text}");
    assert!(!text.contains("--cli-help"), "stdout was: {text}");
    assert!(!text.contains("--help"), "stdout was: {text}");
}

#[test]
fn long_cli_help_flag_is_rejected() {
    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .arg("--cli-help")
        .assert()
        .failure()
        .get_output()
        .stderr
        .clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(
        text.contains("unexpected argument '--cli-help'"),
        "stderr was: {text}"
    );
}

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
fn git_status_and_coloring_for_direct_file_path() {
    let repo = init_repo();
    let file_path = repo.path().join("tracked.txt");
    fs::write(&file_path, "v1\n").expect("write tracked");
    git(repo.path(), &["add", "tracked.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "init"]);

    fs::write(&file_path, "v2\n").expect("update tracked");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["-l", "--color=never", "tracked.txt"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(
        text.starts_with("M"),
        "long output should start with git status M: {text:?}"
    );

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--color=always", "tracked.txt"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).expect("utf8 output");
    assert!(
        text.contains("\u{1b}[33mtracked.txt\u{1b}[39m"),
        "output should color modified file yellow: {text:?}"
    );
}

#[test]
fn git_ls_files_selectors_are_parsed() {
    let dir = tempfile::TempDir::new().unwrap();
    for flag in &[
        "--cached",
        "-c",
        "--stage",
        "--staged",
        "--deleted",
        "--modified",
        "-m",
        "--others",
        "-o",
        "--ignored",
        "--include-ignored",
    ] {
        let mut cmd = Command::cargo_bin("glas").expect("binary");
        cmd.current_dir(dir.path()).arg(flag).assert().success();
    }
}

#[test]
fn git_ls_files_selectors_behavior() {
    let repo = init_repo();

    fs::write(repo.path().join("cached.txt"), "cached\n").expect("write");
    fs::write(repo.path().join("modified.txt"), "modified\n").expect("write");
    fs::write(repo.path().join("deleted.txt"), "deleted\n").expect("write");
    fs::write(repo.path().join(".gitignore"), "ignored.txt\n").expect("write");

    git(
        repo.path(),
        &[
            "add",
            "cached.txt",
            "modified.txt",
            "deleted.txt",
            ".gitignore",
        ],
    );
    git(repo.path(), &["commit", "-q", "-m", "init"]);

    fs::write(repo.path().join("modified.txt"), "modified v2\n").expect("write");
    fs::remove_file(repo.path().join("deleted.txt")).expect("remove");
    fs::write(repo.path().join("untracked.txt"), "untracked\n").expect("write");
    fs::write(repo.path().join("ignored.txt"), "ignored\n").expect("write");

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("cached.txt"));
    assert!(text.contains("modified.txt"));
    assert!(!text.contains("deleted.txt"));
    assert!(text.contains("untracked.txt"));
    assert!(!text.contains("ignored.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--cached", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("cached.txt"));
    assert!(text.contains("modified.txt"));
    assert!(text.contains("deleted.txt"));
    assert!(!text.contains("untracked.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--deleted", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("cached.txt"));
    assert!(!text.contains("modified.txt"));
    assert!(text.contains("deleted.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--modified", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("cached.txt"));
    assert!(text.contains("modified.txt"));
    assert!(text.contains("deleted.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--others", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("cached.txt"));
    assert!(text.contains("untracked.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--ignored", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("cached.txt"));
    assert!(text.contains("ignored.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--others", "--include-ignored", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("untracked.txt"));
    assert!(text.contains("ignored.txt"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--stage", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("100644"));
    assert!(text.contains("cached.txt"));
}

#[test]
fn git_ls_files_flatten_all_behavior() {
    let repo = init_repo();
    fs::create_dir_all(repo.path().join("dir1/dir2/dir3")).unwrap();
    fs::write(repo.path().join("dir1/dir2/dir3/file.txt"), "hello\n").unwrap();
    git(repo.path(), &["add", "dir1/dir2/dir3/file.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "add nested file"]);

    fs::write(repo.path().join("dir1/dir2/dir3/file.txt"), "hello v2\n").unwrap();

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--modified", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(!text.contains("dir1/dir2/dir3/file.txt"));
    assert!(text.contains("dir1"));

    let mut cmd = Command::cargo_bin("glas").expect("binary");
    let output = cmd
        .current_dir(repo.path())
        .args(["--modified", "--flatten=all", "--color=never"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("dir1/dir2/dir3/file.txt"));
}
