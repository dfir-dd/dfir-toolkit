use std::{
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};

use assert_cmd::Command;
use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Accessed, Modified, Changed, Created};

#[test]
fn test_x64dbg() {
    let mut cmd = Command::cargo_bin("lnk2bodyfile").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("lnk2bodyfile");
    data_path.push("x64dbg.lnk");

    let result = cmd.arg(data_path).ok();
    if result.is_err() {
        println!("{}", result.as_ref().err().unwrap());
    }

    assert!(result.is_ok());

    // parse the result as bodyfile 😈
    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    let mut lines_iterator = reader.lines();
    let first_line = lines_iterator.next().unwrap().unwrap();

    let bfline = Bodyfile3Line::try_from(&first_line[..]).unwrap();
    assert_eq!(bfline.get_name(), r#"C:\Program Files\x64dbg\release\x64\x64dbg.exe - (referred to by "x64dbg.lnk")"#);
    assert_eq!(*bfline.get_size(), 172768);
    assert_eq!(*bfline.get_atime(), Accessed::from(1695724808));
    assert_eq!(*bfline.get_mtime(), Modified::from(1695724422));
    assert_eq!(*bfline.get_ctime(), Changed::default());
    assert_eq!(*bfline.get_crtime(), Created::from(1695250410));

    assert!(lines_iterator.next().is_none());
}
