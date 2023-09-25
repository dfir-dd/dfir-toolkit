use chrono::offset::TimeZone;
use chrono::{LocalResult, NaiveDateTime};
use chrono_tz::Tz;
use clap::ValueEnum;
use clio::Input;
use strum_macros::Display;

use super::bodyfile::{BodyfileDecoder, BodyfileReader, BodyfileSorter};
use super::cli::Cli;
use super::error::MactimeError;
use super::filter::{Consumer, Joinable, Provider, RunOptions, Sorter};
use super::output::{CsvOutput, JsonSorter, TxtOutput};
use super::stream::StreamReader;

#[derive(ValueEnum, Clone, Display)]
enum InputFormat {
    #[strum(serialize = "bodyfile")]
    Bodyfile,

    #[cfg(feature = "elastic")]
    #[strum(serialize = "json")]
    Json,
}

#[derive(ValueEnum, Clone, Display)]
pub (crate) enum OutputFormat {
    #[strum(serialize = "csv")]
    Csv,

    #[strum(serialize = "txt")]
    Txt,

    #[strum(serialize = "json")]
    Json,

    #[cfg(feature = "elastic")]
    #[strum(serialize = "elastic")]
    Elastic,
}

//#[derive(Builder)]
pub struct Mactime2Application {
    format: OutputFormat,
    bodyfile: Input,
    src_zone: Tz,
    dst_zone: Tz,
    strict_mode: bool,
}

impl Mactime2Application {
    fn create_sorter(
        &self,
        decoder: &mut BodyfileDecoder,
    ) -> Box<dyn Sorter<Result<(), MactimeError>>> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
            src_zone: self.src_zone,
        };

        if matches!(self.format, OutputFormat::Json) {
            Box::new(JsonSorter::with_receiver(decoder.get_receiver(), options))
        } else {
            let mut sorter =
                BodyfileSorter::default().with_receiver(decoder.get_receiver(), options);

            sorter = sorter.with_output(match self.format {
                OutputFormat::Csv => Box::new(CsvOutput::new(self.src_zone, self.dst_zone)),
                OutputFormat::Txt => Box::new(TxtOutput::new(self.src_zone, self.dst_zone)),
                _ => panic!("invalid execution path"),
            });
            Box::new(sorter)
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
            src_zone: self.src_zone,
        };

        let mut reader = <BodyfileReader as StreamReader<String, ()>>::from(self.bodyfile.clone())?;
        let mut decoder = BodyfileDecoder::with_receiver(reader.get_receiver(), options);
        let mut sorter = self.create_sorter(&mut decoder);
        sorter.run();

        let _ = reader.join();
        let _ = decoder.join();
        sorter.join().unwrap()?;
        Ok(())
    }

    pub fn format_date(unix_ts: i64, src_zone: &Tz, dst_zone: &Tz) -> String {
        if unix_ts >= 0 {
            let src_timestamp = match src_zone
                .from_local_datetime(&NaiveDateTime::from_timestamp_opt(unix_ts, 0).unwrap())
            {
                LocalResult::None => {
                    return "INVALID DATETIME".to_owned();
                }
                LocalResult::Single(t) => t,
                LocalResult::Ambiguous(t1, _t2) => t1,
            };
            let dst_timestamp = src_timestamp.with_timezone(dst_zone);
            dst_timestamp.to_rfc3339()
        } else {
            "0000-00-00T00:00:00+00:00".to_owned()
        }
    }
}

impl From<Cli> for Mactime2Application {
    fn from(cli: Cli) -> Self {
        let format = match cli.output_format {
            Some(f) => f,
            None => {
                if cli.csv_format {
                    OutputFormat::Csv
                } else if cli.json_format {
                    OutputFormat::Json
                } else {
                    OutputFormat::Txt
                }
            }
        };

        Self {
            format,
            bodyfile: cli.input_file,
            src_zone: cli.src_zone.into_tz().unwrap(),
            dst_zone: cli.dst_zone.into_tz().unwrap(),
            strict_mode: cli.strict_mode,
        }
    }
}
