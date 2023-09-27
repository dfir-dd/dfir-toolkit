use std::io::{Cursor, BufReader, BufRead};

use assert_cmd::Command;

const SAMPLE_TIMELINE: &str = r#"1693411717|REG|||App Paths - protocolhandler.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\protocolhandler.exe
1693411717|REG|||App Paths - sdxhelper.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SDXHelper.exe
1693411717|REG|||App Paths - selfcert.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SELFCERT.exe
1693411581|REG|||App Paths - msaccess.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop.Access_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\MSACCESS.exe"#;

#[test]
fn ts2date_simple() {
    const SAMPLE_TIMELINE_OUT: &str = r#"2023-08-30T16:08:37+00:00|REG|||App Paths - protocolhandler.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\protocolhandler.exe
2023-08-30T16:08:37+00:00|REG|||App Paths - sdxhelper.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SDXHelper.exe
2023-08-30T16:08:37+00:00|REG|||App Paths - selfcert.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SELFCERT.exe
2023-08-30T16:06:21+00:00|REG|||App Paths - msaccess.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop.Access_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\MSACCESS.exe
"#;

    let mut cmd = Command::cargo_bin("ts2date").unwrap();
    let result = cmd.write_stdin(SAMPLE_TIMELINE).ok();
    assert!(result.is_ok());

    assert_eq!(
        SAMPLE_TIMELINE_OUT,
        String::from_utf8(result.unwrap().stdout).unwrap()
    );
}

#[test]
fn ts2date_utc2berlin() {
    const SAMPLE_TIMELINE_OUT: &str = r#"2023-08-30T18:08:37+02:00|REG|||App Paths - protocolhandler.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\protocolhandler.exe
2023-08-30T18:08:37+02:00|REG|||App Paths - sdxhelper.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SDXHelper.exe
2023-08-30T18:08:37+02:00|REG|||App Paths - selfcert.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SELFCERT.exe
2023-08-30T18:06:21+02:00|REG|||App Paths - msaccess.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop.Access_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\MSACCESS.exe
"#;

    let mut cmd = Command::cargo_bin("ts2date").unwrap();
    let result = cmd
        .arg("-f")
        .arg("UTC")
        .arg("-t")
        .arg("Europe/Berlin")
        .write_stdin(SAMPLE_TIMELINE)
        .ok();
    assert!(result.is_ok());

    assert_eq!(
        SAMPLE_TIMELINE_OUT,
        String::from_utf8(result.unwrap().stdout).unwrap()
    );
}

#[test]
fn ts2date_berlin2utc() {
    const SAMPLE_TIMELINE_OUT: &str = r#"2023-08-30T14:08:37+00:00|REG|||App Paths - protocolhandler.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\protocolhandler.exe
2023-08-30T14:08:37+00:00|REG|||App Paths - sdxhelper.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SDXHelper.exe
2023-08-30T14:08:37+00:00|REG|||App Paths - selfcert.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\SELFCERT.exe
2023-08-30T14:06:21+00:00|REG|||App Paths - msaccess.exe - C:\Program Files\WindowsApps\Microsoft.Office.Desktop.Access_16051.16626.20170.0_x86__8wekyb3d8bbwe\Office16\MSACCESS.exe
"#;

    let mut cmd = Command::cargo_bin("ts2date").unwrap();
    let result = cmd
        .arg("-f")
        .arg("Europe/Berlin")
        .arg("-t")
        .arg("UTC")
        .write_stdin(SAMPLE_TIMELINE)
        .ok();
    assert!(result.is_ok());

    assert_eq!(
        SAMPLE_TIMELINE_OUT,
        String::from_utf8(result.unwrap().stdout).unwrap()
    );
}

#[test]
fn ts2date_list1() {
    let mut cmd = Command::cargo_bin("ts2date").unwrap();
    let result = cmd.arg("-f").arg("list").ok();
    assert!(result.is_ok());

    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    assert!(reader.lines().map_while(Result::ok).any(|f| f == "Europe/Berlin"));
}

#[test]
fn ts2date_list2() {
    let mut cmd = Command::cargo_bin("ts2date").unwrap();
    let result = cmd.arg("-t").arg("list").ok();
    assert!(result.is_ok());

    let reader = BufReader::new(Cursor::new(result.unwrap().stdout));
    assert!(reader.lines().map_while(Result::ok).any(|f| f == "Europe/Berlin"));
}