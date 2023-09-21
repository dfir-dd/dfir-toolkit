use anyhow::Result;
use clio::Input;
use evtx::{EvtxParser, SerializedEvtxRecord};
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use serde_json::Value;
use ouroboros::self_referencing;

pub (crate) struct EvtxFile (Input);

#[self_referencing()]
pub (crate) struct EvtxFileIterator {
    parser: EvtxParser<Input>,
    
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

impl IntoIterator for EvtxFile {
    type Item = SerializedEvtxRecord<Value>;

    type IntoIter = EvtxFileIterator;

    fn into_iter(self) -> Self::IntoIter {
        let parser = EvtxParser::from_read_seek(self.0).expect("unable to create parser");

        EvtxFileIteratorBuilder {
            parser,
            inner_iterator_builder: |parser: &mut EvtxParser<Input>| Box::new(parser.records_json_value())
        }.build()
    }
}

impl From<Input> for EvtxFile {
    fn from(file: Input) -> Self {
        Self(file)
    }
}


impl EvtxFile {
    pub (crate) fn count_records(&self) -> Result<usize> {
        let mut parser = EvtxParser::from_read_seek(self.0.clone())?;
        Ok(parser.serialized_records(|r| r.and(Ok(()))).count())
    }

    pub (crate) fn create_progress_bar(&self) -> Result<ProgressBar> {
        let count = self.count_records()?;

        let bar = ProgressBar::new(count as u64);
        let target = ProgressDrawTarget::stderr_with_hz(10);
        bar.set_draw_target(target);

        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9}({percent}%) {msg}")?
            .progress_chars("##-");
        bar.set_style(progress_style);

        Ok(bar)
    }
}