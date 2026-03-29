use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn test_file() {
    let mut cmd = Command::cargo_bin("cat").unwrap();

    cmd.arg("tests/input.txt")
        .assert()
        .success()
        .stdout(predicate::eq("hello\nworld\n"));
}

#[test]
fn test_files() {
    let mut cmd = Command::cargo_bin("cat").unwrap();

    cmd.args(["tests/input.txt", "tests/input.txt", "tests/input.txt"])
        .assert()
        .success()
        .stdout(predicate::eq("hello\nworld\nhello\nworld\nhello\nworld\n"));
}

#[test]
fn test_stdin() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    let input = b"Hello\nWorld";
    cmd.write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq("Hello\nWorld"));
}

#[test]
fn test_missing_file() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("does not exist . txt")
        .assert()
        .failure()
        .stderr(predicates::str::contains("No such file"));
}

#[test]
fn test_missing_file_and_real_file() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.args(["nope.txt", "tests/input.txt"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("No such file"))
        .stdout(predicate::eq("hello\nworld\n"));
}

#[test]
fn test_empty_file() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.arg("tests/empty.txt")
        .assert()
        .success()
        .stdout(predicate::eq(""));
}

#[test]
fn test_big_files() {
    let mut cmd = Command::cargo_bin("cat").unwrap();
    cmd.args(["tests/big.txt", "tests/huge.txt"])
        .assert()
        .success();
}
