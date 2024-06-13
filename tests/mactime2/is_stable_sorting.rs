use assert_cmd::Command;
use std::io::{BufReader, Cursor};

/// tests if the result of `mactime2` is a stable sort, i.e. if pos(a)<=pos(b) then sorted_pos(a) <= sorted_pos(b)
#[test]
fn is_stable_sorted() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();

    let sample_bodyfile =
        "0|a|1703937|d/drwxr-xr-x|0|0|4096|1661774613|1661774613|1661774613|1661774613
0|b|11|d/drwx------|0|0|16384|1661774613|1661774613|1661774613|1661774613
0|c|11|d/drwx------|0|0|16384|1661774613|1661774613|1661774613|1661774613";

    let result = cmd
        .arg("-d")
        .arg("-b")
        .arg("-")
        .write_stdin(sample_bodyfile)
        .ok();
    assert!(result.is_ok());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(Cursor::new(result.unwrap().stdout)));
    let names: Vec<_> = reader
        .records()
        .filter_map(Result::ok)
        .map(|record| record.get(7).unwrap().to_owned())
        .collect();
    assert_eq!(names, vec!["a", "b", "c"]);
}
