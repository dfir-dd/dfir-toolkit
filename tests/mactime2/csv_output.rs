use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
};

use assert_cmd::Command;

/// tests if the result of `mactime2` is always sorted
#[test]
fn csv_output() {
    let mut cmd = Command::cargo_bin("mactime2").unwrap();
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push("mactime2");
    data_path.push("csv_test.bodyfile");

    let result = cmd.arg("-d").arg("-b").arg(data_path).ok();
    assert!(result.is_ok());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(Cursor::new(result.unwrap().stdout)));
    let first_line = reader.records().next().unwrap().unwrap();

    assert_eq!(first_line.get(7).unwrap(), r##"{"activity_id":null,"channel_name":"Microsoft-Windows-WER-PayloadHealth/Operational","computer":"WIN-J56D9ENVG6H","custom_data":{"EventData":{"#attributes":{"Name":"WER_PAYLOAD_HEALTH_FAIL"},"BytesUploaded":0,"HttpExchangeResult":2147954402,"PayloadSize":4569,"Protocol":"Watson","RequestStatusCode":0,"ServerName":"umwatson.events.data.microsoft.com","Stage":"s1event","TransportHr":2147954402,"UploadDuration":21094}},"event_id":2,"event_record_id":1,"level":4,"provider_name":"Microsoft-Windows-WER-PayloadHealth","timestamp":"2022-11-16T08:26:43.409044Z"}"##);
}
