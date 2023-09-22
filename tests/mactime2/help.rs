use assert_cmd::Command;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    cmd.arg("--help").assert().success();
}