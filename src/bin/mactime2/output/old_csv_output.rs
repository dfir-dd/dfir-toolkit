use std::io::Write;

use chrono_tz::Tz;
use dfir_toolkit::common::ForensicsTimestamp;

use crate::bodyfile::{ListEntry, Mactime2Writer};

pub(crate) struct OldCsvOutput<W>
where
    W: Write + Send,
{
    dst_zone: Tz,
    writer: W,
}

impl<W> OldCsvOutput<W>
where
    W: Write + Send,
{
    pub fn new(writer: W, dst_zone: Tz) -> Self {
        Self { dst_zone, writer }
    }
}

impl<W> Mactime2Writer<W> for OldCsvOutput<W>
where
    W: Write + Send,
{
    fn write_line(&mut self, timestamp: &i64, entry: &ListEntry) -> std::io::Result<()> {
        let timestamp = ForensicsTimestamp::from(*timestamp).with_timezone(self.dst_zone);
        writeln!(
            self.writer,
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

    fn into_writer(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use crate::bodyfile::ListEntry;
    use crate::bodyfile::MACBFlags;
    use crate::bodyfile::Mactime2Writer;

    use super::OldCsvOutput;
    use chrono::DateTime;
    use chrono_tz::Tz;
    use chrono_tz::TZ_VARIANTS;
    use dfir_toolkit::common::bodyfile::Bodyfile3Line;
    use std::io::Cursor;
    use std::io::{BufRead, BufReader};
    use std::sync::Arc;

    fn random_tz() -> Tz {
        let index = rand::random::<usize>() % TZ_VARIANTS.len();
        TZ_VARIANTS[index]
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_correct_ts_UTC() {
        for _ in 1..10 {
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts.into());
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line),
            };

            let mut output = OldCsvOutput::new(Cursor::new(vec![]), Tz::UTC);

            output.write_line(&unix_ts, &entry).unwrap();
            let mut output = BufReader::new(Cursor::new(output.into_writer().into_inner())).lines();
            let out_line = output.next().unwrap().unwrap();

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
            let mut output = OldCsvOutput::new(Cursor::new(vec![]), tz);

            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts.into());
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line),
            };

            output.write_line(&unix_ts, &entry).unwrap();
            let mut output = BufReader::new(Cursor::new(output.into_writer().into_inner())).lines();
            let out_line = output.next().unwrap().unwrap();

            let out_ts = out_line.split(',').next().unwrap();
            let rfc3339 = match DateTime::parse_from_rfc3339(out_ts) {
                Ok(ts) => ts,
                Err(e) => return Err(format!("error while parsing '{}': {}", out_ts, e)),
            };
            let calculated_ts = rfc3339.timestamp();
            assert_eq!(
                unix_ts, calculated_ts,
                "Timestamp {unix_ts} converted to '{out_ts}' and back to {calculated_ts}",
            );
        }
        Ok(())
    }
}
