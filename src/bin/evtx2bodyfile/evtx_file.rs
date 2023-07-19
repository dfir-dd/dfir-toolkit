use std::{path::PathBuf, fs::File};
use anyhow::Result;
use evtx::{EvtxParser, SerializedEvtxRecord, err::EvtxError};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use serde_json::Value;
use ouroboros::self_referencing;

pub (crate) struct EvtxFile {
    filename: String,
    fp: PathBuf,
}

#[self_referencing()]
pub (crate) struct EvtxFileIterator {
    parser: EvtxParser<File>,
    
    #[borrows(mut parser)]
    #[not_covariant]
    inner_iterator: Box <dyn Iterator<Item = evtx::err::Result<SerializedEvtxRecord<Value>>> + 'this>
}

impl Iterator for EvtxFileIterator {
    type Item = SerializedEvtxRecord<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.with_inner_iterator_mut(|iter| {
            loop {
                match iter.next() {
                    None => return None,
                    Some(Ok(record)) => return Some(record),
                    Some(Err(why)) => log::warn!("{}", why)
                }
            }
        })
    }
}

impl<'a> IntoIterator for &'a EvtxFile {
    type Item = SerializedEvtxRecord<Value>;

    type IntoIter = EvtxFileIterator;

    fn into_iter(self) -> Self::IntoIter {
        let parser = EvtxParser::from_path(&self.fp).expect("unable to create parser");

        EvtxFileIteratorBuilder {
            parser,
            inner_iterator_builder: |parser: &mut EvtxParser<File>| Box::new(parser.records_json_value())
        }.build()
    }
}

impl TryFrom<&str> for EvtxFile {
    type Error = EvtxError;
    fn try_from(file: &str) -> std::result::Result<Self, Self::Error> {
        let fp = PathBuf::from(file);
        let filename = fp.file_name().unwrap().to_str().unwrap().to_owned();

        Ok(Self {
            filename,
            fp: PathBuf::from(file),
        })
    }

}


impl EvtxFile {
    pub (crate) fn count_records(&self) -> Result<usize> {
        let mut parser = EvtxParser::from_path(&self.fp)?;
        Ok(parser.serialized_records(|r| r.and(Ok(()))).count())
    }

    pub (crate) fn create_progress_bar(&self) -> Result<ProgressBar> {
        let count = self.count_records()?;

        let bar = ProgressBar::new(count as u64);
        let target = ProgressDrawTarget::stderr_with_hz(10);
        bar.set_draw_target(target);
        bar.set_message(self.filename.clone());

        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9}({percent}%) {msg}")?
            .progress_chars("##-");
        bar.set_style(progress_style);

        Ok(bar)
    }
}