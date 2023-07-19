
use crate::{bf_data::*, evtx_file::EvtxFile};
use anyhow::Result;
use clap::Parser;
use evtx::SerializedEvtxRecord;
use getset::Getters;
use serde_json::Value;

#[derive(Parser, Clone, Getters)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Evtx2BodyfileApp {
    /// names of the evtx files
    evtx_files: Vec<String>,

    /// output json for elasticsearch instead of bodyfile
    #[clap(short('J'), long("json"))]
    json_output: bool,

    /// fail upon read error
    #[clap(short('S'), long("strict"))]
    strict: bool,

    #[clap(flatten)]
    #[getset(get = "pub (crate)")]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Evtx2BodyfileApp {
    pub(crate) fn handle_evtx_files(&self) -> Result<()> {
        for file in self.evtx_files.iter() {
            self.handle_evtx_file((&file[..]).try_into()?);
        }
        Ok(())
    }

    fn handle_evtx_file(&self, evtx_file: EvtxFile) {       
        let bar = evtx_file.create_progress_bar().unwrap();
        for value in evtx_file.into_iter() {
            if let Err(why) = self.print_record(&value) {
                if self.strict {
                    log::error!("Error while reading record: {why}");
                    bar.finish_and_clear();
                    return;
                } else {
                    log::warn!("Error while reading record: {why}");
                }
            }
            bar.inc(1);
        }
        bar.finish_and_clear();
    }

    fn print_record(&self, record: &SerializedEvtxRecord<Value>) -> Result<()> {
        let mut bf_data: BfData = record.try_into()?;
        bf_data.set_enable_json_output(self.json_output);

        match TryInto::<String>::try_into(bf_data) {
            Err(why) => log::warn!("{}", why),
            Ok(line) => println!("{}", line),
        }
        Ok(())
    }
}
