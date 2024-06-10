use std::path::PathBuf;

use assert_cmd::Command;

#[test]
fn scan_testhive() {
    let mut cmd = Command::cargo_bin("hivescan").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("hivescan");
    data_path.push("testhive");
    cmd.arg(data_path).assert().success();
}