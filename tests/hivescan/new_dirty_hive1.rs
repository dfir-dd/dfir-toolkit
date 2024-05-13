use std::path::PathBuf;

use assert_cmd::Command;

#[test]
fn scan_new_dirty_hive1() {
    let mut cmd = Command::cargo_bin("hivescan").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("hivescan");
    data_path.push("NewDirtyHive1");
    data_path.push("NewDirtyHive");
    cmd.arg(data_path).assert().success();
}