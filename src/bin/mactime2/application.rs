use chrono_tz::Tz;
use clap::ValueEnum;
use clio::Input;
use strum_macros::Display;

use crate::output::OldCsvOutput;

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
pub(crate) enum OutputFormat {
    /// Comma-Separated Values compliant to RFC 4180
    #[strum(serialize = "csv")]
    Csv,

    /// legacy text format, inherited from the old mactime
    #[strum(serialize = "txt")]
    Txt,

    /// Javascript Object Notation
    #[strum(serialize = "json")]
    Json,

    /// JSON-format to be used by elasticsearch
    #[cfg(feature = "elastic")]
    #[strum(serialize = "elastic")]
    Elastic,

    /// Use the old (non RFC compliant) CSV format that was used by legacy mactime.
    #[strum(serialize = "old-csv")]
    OldCsv,
}

pub struct Mactime2Application {
    format: OutputFormat,
    bodyfile: Input,
    dst_zone: Tz,
    show_headers: bool,
    strict_mode: bool,
}

impl Mactime2Application {
    fn create_sorter(
        &self,
        decoder: &mut BodyfileDecoder,
    ) -> Box<dyn Sorter<Result<(), MactimeError>>> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
        };

        if matches!(self.format, OutputFormat::Json) {
            Box::new(JsonSorter::with_receiver(decoder.get_receiver(), options))
        } else {
            let mut sorter =
                BodyfileSorter::default().with_receiver(decoder.get_receiver(), options);

            sorter = sorter.with_output(match self.format {
                OutputFormat::OldCsv => {
                    Box::new(OldCsvOutput::new(std::io::stdout(), self.dst_zone))
                }

                OutputFormat::Csv => Box::new(CsvOutput::new(
                    std::io::stdout(),
                    self.dst_zone,
                    self.show_headers,
                )),
                OutputFormat::Txt => Box::new(TxtOutput::new(std::io::stdout(), self.dst_zone)),
                _ => panic!("invalid execution path"),
            });
            Box::new(sorter)
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let options = RunOptions {
            strict_mode: self.strict_mode,
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
            dst_zone: cli.dst_zone.into_tz().unwrap(),
            show_headers: cli.show_headers,
            strict_mode: cli.strict_mode,
        }
    }
}
