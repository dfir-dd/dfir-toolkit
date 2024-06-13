use std::io::Write;

use chrono_tz::Tz;
use csv::WriterBuilder;
use dfir_toolkit::common::ForensicsTimestamp;
use serde::Serialize;

use crate::bodyfile::{ListEntry, MACBFlags, Mactime2Writer};

pub(crate) struct CsvOutput<W>
where
    W: Write + Send,
{
    dst_zone: Tz,
    writer: csv::Writer<W>,
}

pub const CSV_DELIMITER: u8 = b',';

impl<W> CsvOutput<W>
where
    W: Write + Send,
{
    pub fn new(writer: W, dst_zone: Tz) -> Self {
        Self {
            dst_zone,
            writer: WriterBuilder::new()
                .delimiter(CSV_DELIMITER)
                .has_headers(false)
                .from_writer(writer),
        }
    }
    #[allow(dead_code)]
    pub fn with_writer(mut self, writer: W) -> Self
    where
        W: Write + Send + 'static,
    {
        self.writer = WriterBuilder::new().from_writer(writer);
        self
    }
}

impl<W> Mactime2Writer<W> for CsvOutput<W>
where
    W: Write + Send,
{
    fn write_line(&mut self, timestamp: &i64, entry: &ListEntry) -> std::io::Result<()> {
        let csv_line = CsvLine {
            timestamp: ForensicsTimestamp::new(*timestamp, self.dst_zone),
            size: entry.line.get_size(),
            flags: entry.flags,
            mode: entry.line.get_mode_as_string(),
            uid: entry.line.get_uid(),
            gid: entry.line.get_gid(),
            inode: entry.line.get_inode(),
            name: entry.line.get_name(),
        };
        self.writer.serialize(csv_line)?;
        Ok(())
    }

    fn into_writer(self) -> W {
        self.writer.into_inner().unwrap()
    }
}

#[derive(Serialize)]
struct CsvLine<'e> {
    timestamp: ForensicsTimestamp,
    size: &'e u64,
    flags: MACBFlags,
    mode: &'e str,
    uid: &'e u64,
    gid: &'e u64,
    inode: &'e str,
    name: &'e str,
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
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::Cursor;
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

            let mut output = CsvOutput::new(Cursor::new(vec![]), Tz::UTC);
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
            let unix_ts = rand::random::<u32>() as i64;
            let bf_line = Bodyfile3Line::new().with_crtime(unix_ts.into());
            let entry = ListEntry {
                flags: MACBFlags::B,
                line: Arc::new(bf_line),
            };

            let mut output = CsvOutput::new(Cursor::new(vec![]), tz);
            let delimiter: char = crate::output::CSV_DELIMITER.into();
            output.write_line(&unix_ts, &entry).unwrap();
            let mut output = BufReader::new(Cursor::new(output.into_writer().into_inner())).lines();
            let out_line = output.next().unwrap().unwrap();

            let out_ts = out_line.split(delimiter).next().unwrap();
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
