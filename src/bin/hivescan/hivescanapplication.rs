use crate::cli::Cli;
use crate::regtreeentry::RegTreeEntry;
use anyhow::Result;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use indicatif::{ProgressBar, ProgressStyle};
use nt_hive2::*;
use std::{io::Read, io::Seek};

use crate::regtreebuilder::RegTreeBuilder;

pub(crate) struct HiveScanApplication<RS>
where
    RS: Read + Seek,
{
    #[allow(dead_code)]
    cli: Cli,

    root_offset: Offset,
    hive: Option<Hive<RS, CleanHive>>,
}

impl<RS> HiveScanApplication<RS>
where
    RS: Read + Seek,
{
    pub fn new(cli: Cli, hive: Hive<RS, CleanHive>) -> Self {
        Self {
            cli,
            root_offset: hive.root_cell_offset(),
            hive: Some(hive),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        assert!(self.hive.is_some());

        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>9}/{len:9}({percent}%) {msg}")
            .unwrap();
        let bar = ProgressBar::new(self.hive.as_ref().unwrap().data_size().into());
        bar.set_style(progress_style);
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
                .with_ctime(entry.nk().timestamp().into());
            println!("{}", bf_line);
        } else if entry.is_deleted() || force_print {
            println!(
                "[{}]; last change at {}, found at offset 0x{:x}",
                path,
                entry.nk().timestamp().to_rfc3339(),
                entry.offset().0 + BASEBLOCK_SIZE as u32
            );
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
