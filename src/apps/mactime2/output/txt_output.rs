use chrono_tz::Tz;
use std::cell::RefCell;

use crate::apps::mactime2::{bodyfile::{Mactime2Writer, ListEntry}, Mactime2Application};

pub struct TxtOutput {
    src_zone: Tz, dst_zone: Tz,
    last_ts: (RefCell<i64>, RefCell<String>),
    empty_ts: RefCell<String>
}

impl TxtOutput {
    pub fn new(src_zone: Tz, dst_zone: Tz) -> Self {
        Self {
            src_zone, dst_zone,
            last_ts: (RefCell::new(i64::MIN), RefCell::new("".to_owned())),
            empty_ts: RefCell::new("                         ".to_owned())
        }
    }
}

impl Mactime2Writer for TxtOutput {
    fn fmt(&self, timestamp: &i64, entry: &ListEntry) -> String {
        let ts = if *timestamp != *self.last_ts.0.borrow() {
            *self.last_ts.1.borrow_mut() = Mactime2Application::format_date(*timestamp, &self.src_zone, &self.dst_zone);
            *self.last_ts.0.borrow_mut() = *timestamp;
            self.last_ts.1.borrow()
        } else {
            self.empty_ts.borrow()
        };
        format!(
            "{} {:>8} {} {:<12} {:<7} {:<7} {} {}",
            ts,
            entry.line.get_size(),
            entry.flags,
            entry.line.get_mode(),
            entry.line.get_uid(),
            entry.line.get_gid(),
            entry.line.get_inode(),
            entry.line.get_name()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TxtOutput;
    use chrono::DateTime;
    use chrono_tz::TZ_VARIANTS;
    use chrono_tz::Tz;
    use std::sync::Arc;
    use bodyfile::Bodyfile3Line;
    use crate::apps::mactime2::bodyfile::{MACBFlags, ListEntry, Mactime2Writer};

    fn random_tz() -> Tz {
        let index = rand::random::<usize>() % TZ_VARIANTS.len();
        TZ_VARIANTS[index]
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_correct_ts_UTC() {
        let output = TxtOutput::new(Tz::UTC, Tz::UTC);
        for _ in 1..10 {
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts);
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line)
            };

            let out_line = output.fmt(&unix_ts, &entry);
            let out_line2 = output.fmt(&unix_ts, &entry);
            assert!(out_line2.starts_with(' '));
            
            let out_ts = out_line.split(' ').next().unwrap();
            let rfc3339 = DateTime::parse_from_rfc3339(out_ts).expect(out_ts);
            assert_eq!(unix_ts, rfc3339.timestamp(), "Timestamp {} converted to '{}' and back to {}", unix_ts, out_ts, rfc3339.timestamp());
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_correct_ts_random_tz() -> Result<(), String> {
        for _ in 1..100 {
            let tz = random_tz();
            let output = TxtOutput::new(tz, tz);
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts);
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line)
            };

            let out_line = output.fmt(&unix_ts, &entry);
            let out_line2 = output.fmt(&unix_ts, &entry);
            assert!(out_line2.starts_with(' '));

            let out_ts = out_line.split(' ').next().unwrap();
            let rfc3339 = match DateTime::parse_from_rfc3339(out_ts) {
                Ok(ts) => ts,
                Err(e) => return Err(format!("error while parsing '{}': {}", out_ts, e))
            };
            let offset = rfc3339.offset().local_minus_utc() as i64;
            let calculated_ts = rfc3339.timestamp() + offset;
            assert_eq!(unix_ts, calculated_ts, "Timestamp {} converted to '{}' and back to {} (offset was {}s)", unix_ts, out_ts, calculated_ts, offset);
        }
        Ok(())
    }
}
