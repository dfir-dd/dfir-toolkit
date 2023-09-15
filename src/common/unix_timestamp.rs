use chrono::NaiveDateTime;


pub struct UnixTimestamp {
}

impl UnixTimestamp {
    pub fn ts2date(ts: i64) -> String {
        let nt = NaiveDateTime::from_timestamp_opt(ts, 0);
        match nt {
            Some(_) => nt.unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
            None => panic!("Something went wrong"),
        }
    }
}