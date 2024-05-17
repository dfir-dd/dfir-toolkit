use chrono_tz::Tz;
use dfir_toolkit::common::ForensicsTimestamp;

use crate::bodyfile::{ListEntry, Mactime2Writer};

pub(crate) struct CsvOutput {
    src_zone: Tz,
    dst_zone: Tz,
}

impl CsvOutput {
    pub fn new(src_zone: Tz, dst_zone: Tz) -> Self {
        Self { src_zone, dst_zone }
    }
}

impl Mactime2Writer for CsvOutput {
    fn fmt(&self, timestamp: &i64, entry: &ListEntry) -> String {
        let timestamp = ForensicsTimestamp::new(*timestamp, self.src_zone, self.dst_zone);
        format!(
            "{},{},{},{},{},{},{},\"{}\"",
            timestamp,
            entry.line.get_size(),
            entry.flags,
            entry.line.get_mode_as_string(),
            entry.line.get_uid(),
            entry.line.get_gid(),
            entry.line.get_inode(),
            entry.line.get_name()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bodyfile::ListEntry;
    use crate::bodyfile::MACBFlags;
    use crate::bodyfile::Mactime2Writer;

    use super::CsvOutput;
    use chrono::DateTime;
    use chrono_tz::Tz;
    use chrono_tz::TZ_VARIANTS;
    use dfir_toolkit::common::bodyfile::Bodyfile3Line;
    use std::sync::Arc;

    fn random_tz() -> Tz {
        let index = rand::random::<usize>() % TZ_VARIANTS.len();
        TZ_VARIANTS[index]
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_correct_ts_UTC() {
        let output = CsvOutput::new(Tz::UTC, Tz::UTC);
        for _ in 1..10 {
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts.into());
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line),
            };

            let out_line = output.fmt(&unix_ts, &entry);
            let out_ts = out_line.split(',').next().unwrap();
            let rfc3339 = DateTime::parse_from_rfc3339(out_ts)
                .expect(out_ts)
                .timestamp();
            assert_eq!(
                unix_ts, rfc3339,
                "Timestamp {unix_ts} converted to '{out_ts}' and back to {rfc3339}",
            );
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_correct_ts_random_tz() -> Result<(), String> {
        for _ in 1..100 {
            let tz = random_tz();
            let output = CsvOutput::new(tz, tz);
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts.into());
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line),
            };

            let out_line = output.fmt(&unix_ts, &entry);
            let out_ts = out_line.split(',').next().unwrap();
            let rfc3339 = match DateTime::parse_from_rfc3339(out_ts) {
                Ok(ts) => ts,
                Err(e) => return Err(format!("error while parsing '{}': {}", out_ts, e)),
            };
            let offset = rfc3339.offset().local_minus_utc() as i64;
            let calculated_ts = rfc3339.timestamp() + offset;
            assert_eq!(
                unix_ts, calculated_ts,
                "Timestamp {unix_ts} converted to '{out_ts}' and back to {calculated_ts} (offset was {offset}s)",
            );
        }
        Ok(())
    }
}
