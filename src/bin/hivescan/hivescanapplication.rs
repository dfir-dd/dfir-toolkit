use std::time::Duration;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use nt_hive2::*;
use crate::regtreeentry::RegTreeEntry;
use std::{fs::File, path::PathBuf};
use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use crate::regtreebuilder::RegTreeBuilder;

/// scans a registry hive file for deleted entries
#[derive(Parser)]
#[clap(name=env!("CARGO_BIN_NAME"), author, version)]
pub (crate) struct Args {
    /// name of the file to scan
    pub (crate) hive_file: String,

    /// transaction LOG file(s). This argument can be specified one or two times.
    #[clap(short('L'), long("log"))]
    #[arg(value_parser = validate_file)]
    pub (crate) logfiles: Vec<PathBuf>,

    #[clap(flatten)]
    pub (crate) verbose: clap_verbosity_flag::Verbosity,

    /// output as bodyfile format
    #[clap(short('b'))]
    pub (crate) print_bodyfile: bool,

    /// print help in markdown format
    #[arg(long, hide = true, exclusive=true)]
    pub markdown_help: bool,
}

fn validate_file(s: &str) -> Result<PathBuf, String> {
    let pb = PathBuf::from(s);
    if pb.is_file() && pb.exists() {
        Ok(pb)
    } else {
        Err(format!("unable to read file: '{s}'"))
    }
}

pub (crate) struct HiveScanApplication{

    #[allow(dead_code)]
    cli: Args,

    root_offset: Offset,
    hive: Option<Hive<File, CleanHive>>
}

impl HiveScanApplication {
    pub fn new(cli: Args, hive: Hive<File, CleanHive>) -> Self {
        Self {
            cli,
            root_offset: hive.root_cell_offset(),
            hive: Some(hive) 
        }
    }

    pub fn run(&mut self) -> Result<()> {
        assert!(self.hive.is_some());

        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9}({percent}%) {msg}").unwrap();
        let bar = ProgressBar::new(self.hive.as_ref().unwrap().data_size().into());
        bar.set_style(progress_style);
        bar.enable_steady_tick(Duration::from_millis(100));
        bar.set_message("scanning cells");

        let builder = RegTreeBuilder::from_hive(self.hive.take().unwrap(), |p| bar.set_position(p));
        
        assert!(self.hive.is_none());
        
        for node in builder.root_nodes() {
            if node.borrow().offset() == &self.root_offset {
                // this is the root entry, which we don't print by itself
                for grandchild in node.borrow().children() {
                    self.print_entry("", &grandchild, false);
                }
            } else {
                let parent = format!("/$Orphaned/{:x}", node.borrow().nk().parent.0);
                self.print_entry(&parent, &node.borrow(), false);
            }
        }
        Ok(())
    }

    fn print_entry(&self, path: &str, entry: &RegTreeEntry, force_print: bool) {
        let path = format!("{}/{}", path, entry.nk().name());
    
        if self.cli.print_bodyfile {
            let bf_name = if entry.is_deleted() {
                format!("{} (deleted)", path)
            } else {
                path.clone()
            };

            let bf_line = Bodyfile3Line::new()
                .with_owned_name(bf_name)
                .with_inode(&format!("{:x}", entry.offset().0))
                .with_ctime(entry.nk().timestamp().timestamp());
            println!("{}", bf_line);
        } else if entry.is_deleted() || force_print {
            println!("[{}]; last change at {}, found at offset 0x{:x}", 
                path,
                entry.nk().timestamp().to_rfc3339(),
                entry.offset().0 + BASEBLOCK_SIZE as u32);
            self.print_values_of(entry);
            println!();
        }

    
        for child in entry.children() {
            self.print_entry(&path, &child, entry.is_deleted());
        }
    }

    fn print_values_of(&self, entry: &RegTreeEntry) {
        
        for value in entry.nk().values() {
            println!("\"{}\" = {}", value.name(), value.value());
        }
    }

}
