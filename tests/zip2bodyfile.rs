use std::{
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
};

use assert_cmd::Command;
use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Accessed, Modified, Changed, Created};

#[test]
fn test_hello() {
    do_test_hello(r#"hello.txt, [offset: +01:00]"#, vec![].into_iter());
}

#[test]
fn test_hello_with_archive_name() {
    do_test_hello(r#"hello.txt (in archive hello.zip), [offset: +01:00]"#, vec!["--show-archive-name"].into_iter());
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

    println!("{first_line}");
    let bfline = Bodyfile3Line::try_from(&first_line[..]).unwrap();
    assert_eq!(bfline.get_name(), expected_name);
    assert_eq!(*bfline.get_size(), 12);
    assert_eq!(*bfline.get_atime(), Accessed::from(1709194030));
    assert_eq!(*bfline.get_mtime(), Modified::from(1709194030));
    assert_eq!(*bfline.get_ctime(), Changed::default());
    assert_eq!(*bfline.get_crtime(), Created::from(1709194030));

    assert!(lines_iterator.next().is_none());
}



#[test]
fn test_hello2() {
    do_test_hello2(vec![].into_iter());
}


fn do_test_hello2(args: impl Iterator<Item = &'static str>) {
    let mut cmd = Command::cargo_bin("zip2bodyfile").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("zip2bodyfile");
    data_path.push("hello2.zip");

    let result = cmd.arg(data_path).args(args).ok();
    if result.is_err() {
        println!("{}", result.as_ref().err().unwrap());
    }

    assert!(result.is_ok());

    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    let mut lines_iterator = reader.lines();
    let first_line = lines_iterator.next().unwrap().unwrap();
    
    /* 
    * Archive:  ~/dfir-toolkit/tests/data/zip2bodyfile/hello2.zip
    * c3636a8166a5e10268c90ee3fc37fe44b1b23d80
    *   Length      Date    Time    Name
    * ---------  ---------- -----   ----
    *        0  2024-02-28 20:16   dfir-toolkit-feature-zip2bodyfile/
    *    35149  2024-02-28 20:16   dfir-toolkit-feature-zip2bodyfile/LICENSE
    */

    let mut expected_name = String::from("dfir-toolkit-feature-zip2bodyfile/");
    let mut bfline = Bodyfile3Line::try_from(&first_line[..]).unwrap();
    
    assert_eq!(bfline.get_name(), expected_name.as_str());
    assert_eq!(*bfline.get_size(), 0);
    // 1709151390 = 2024-02-28 20:16:30
    // 1709118990 = 2024-02-28 11:16:30
    assert_eq!(*bfline.get_mtime(), Modified::from(1709151390));

    let second_line = lines_iterator.next().unwrap().unwrap();
    bfline = Bodyfile3Line::try_from(&second_line[..]).unwrap();
    expected_name = String::from("dfir-toolkit-feature-zip2bodyfile/LICENSE");
    assert_eq!(bfline.get_name(), expected_name.as_str());
    assert_eq!(*bfline.get_size(), 35149);
    // 1709151390 = 2024-02-28 20:16:30
    assert_eq!(*bfline.get_mtime(), Modified::from(1709151390));


    assert!(lines_iterator.next().is_none());
}
