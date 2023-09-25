use std::io::{BufReader, Cursor, BufRead};
use lazy_regex::regex;
use assert_cmd::Command;

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

    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    let lines: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(name_of)
        .collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0].as_ref().unwrap(), "a");
    assert_eq!(lines[1].as_ref().unwrap(), "b");
    assert_eq!(lines[2].as_ref().unwrap(), "c");
}

fn name_of(line: String) -> Option<String> {
    let re = regex!(r#""(?P<name>[^"]*)""#);
    let result = re.captures_iter(&line);
    for c in result {
        if let Some(name) = c.name("name") {
            return Some(name.as_str().to_owned())
        }
    }
    None
}