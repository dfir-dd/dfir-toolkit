use std::{
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};

use assert_cmd::Command;
use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Accessed, Modified, Changed, Created};

#[test]
fn test_hello() {
    do_test_hello(r#"hello.txt"#, vec![].into_iter());
}

#[test]
fn test_hello_with_archive_name() {
    do_test_hello(r#"hello.txt (in archive hello.zip)"#, vec!["--show-archive-name"].into_iter());
}

fn do_test_hello(expected_name: &str, args: impl Iterator<Item = &'static str>) {
    let mut cmd = Command::cargo_bin("zip2bodyfile").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("zip2bodyfile");
    data_path.push("hello.zip");

    let result = cmd.arg(data_path).args(args).ok();
    if result.is_err() {
        println!("{}", result.as_ref().err().unwrap());
    }

    assert!(result.is_ok());

    // parse the result as bodyfile 😈
    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    let mut lines_iterator = reader.lines();
    let first_line = lines_iterator.next().unwrap().unwrap();

    let bfline = Bodyfile3Line::try_from(&first_line[..]).unwrap();
    assert_eq!(bfline.get_name(), expected_name);
    assert_eq!(*bfline.get_size(), 12);
    assert_eq!(*bfline.get_atime(), Accessed::default());
    assert_eq!(*bfline.get_mtime(), Modified::from(1709197630));
    assert_eq!(*bfline.get_ctime(), Changed::default());
    assert_eq!(*bfline.get_crtime(), Created::default());

    assert!(lines_iterator.next().is_none());
}