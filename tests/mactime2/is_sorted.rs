use std::{path::PathBuf, io::{BufReader, Cursor, BufRead}};

use assert_cmd::Command;
use more_asserts::assert_le;

#[test]
fn is_sorted() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("mactime2");
    data_path.push("sample.bodyfile");

    let result = cmd.arg("-d").arg("-b").arg(data_path).ok();
    assert!(result.is_ok());

    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    let mut previous_line = None;
    for line in reader.lines().map_while(Result::ok) {
        let comma_index = line.find(',').unwrap();
        assert_eq!(comma_index, "2022-04-18T10:28:59+00:00".len());
        let timestamp = line[..comma_index].to_owned();
        if let Some(pv) = previous_line {
            assert_le!(pv, timestamp);
        }
        previous_line = Some(timestamp);
    }
}
