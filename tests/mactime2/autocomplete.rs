use assert_cmd::Command;

#[test]
fn test_bash() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    cmd.arg("--autocomplete").arg("bash").assert().success();
}

#[test]
fn test_zsh() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    cmd.arg("--autocomplete").arg("zsh").assert().success();
}

#[test]
fn test_fish() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    cmd.arg("--autocomplete").arg("fish").assert().success();
}