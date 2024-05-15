mod cli;

use cli::Cli;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use forensic_rs::prelude::*;
use frnsc_prefetch::prelude::*;
use log::{error, warn};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_cli();

    if cli.prefetch_files().iter().any(|f| !f.path().exists()) {
        anyhow::bail!("some files you specified do not exist");
    }
    if cli.prefetch_files().iter().any(|f| !f.path().is_file()) {
        anyhow::bail!("some paths you specified are no files");
    }

    let vfs = Box::new(StdVirtualFS::new());

    for input in cli.prefetch_files().iter() {
        match input.parent() {
            Some(parent) => {
                let mut fs = ChRootFileSystem::new(parent, vfs.clone());
                if let Some(pf_os_filename) = input.path().file_name() {
                    if let Some(pf_filename) = pf_os_filename.to_str() {
                        let virtual_file = fs.open(Path::new(&pf_filename.to_string()))?;
                        let created = virtual_file
                            .metadata()?
                            .created_opt()
                            .and_then(|t| i64::try_from(*t).ok());
                        let modified = virtual_file
                            .metadata()?
                            .modified_opt()
                            .and_then(|t| i64::try_from(*t).ok());
                        let pf_file = read_prefetch_file(pf_filename, virtual_file)?;

                        pf_file.display_prefetch_file(
                            pf_filename,
                            *cli.include_metrics(),
                            created,
                            modified,
                        )?;
                    } else {
                        error!("invalid Unicode characters in filename: '{pf_os_filename:?}'")
                    }
                } else {
                    warn!("unable to handle directories; you must specify concrete file names");
                }
            }
            None => {
                error!("specified path has no parent: {input}")
            }
        }
    }
    Ok(())
}

trait DisplayPrefetchFile {
    fn display_prefetch_file(
        &self,
        pf_file_name: &str,
        include_metrics: bool,
        created: Option<i64>,
        modified: Option<i64>,
    ) -> anyhow::Result<()>;
}

impl DisplayPrefetchFile for PrefetchFile {
    fn display_prefetch_file(
        &self,
        pf_file_name: &str,
        include_metrics: bool,
        created: Option<i64>,
        modified: Option<i64>,
    ) -> anyhow::Result<()> {
        for time in &self.last_run_times {
            let accessed =
                winstructs::timestamp::WinTimestamp::new(&time.filetime().to_le_bytes())?
                    .to_datetime()
                    .into();

            let mut bf_line = Bodyfile3Line::new()
                .with_owned_name(format!(
                    "Prefetch: run '{}' (run {} times, read from '{pf_file_name}')",
                    self.name, self.run_count
                ))
                .with_atime(accessed);

            if let Some(ts) = created {
                bf_line = bf_line.with_crtime(ts.into());
            }
            if let Some(ts) = modified {
                bf_line = bf_line.with_mtime(ts.into());
            }
            println!("{bf_line}");

            if include_metrics {
                for metric in &self.metrics {
                    let mf = &metric.file;
                    let bf_line = Bodyfile3Line::new()
                        .with_owned_name(format!(
                            "Prefetch: running '{} possibly loaded '{mf}', read from '{pf_file_name}')",
                            self.name
                        ))
                        .with_atime(accessed);
                    println!("{bf_line}");
                }
            }
        }
        Ok(())
    }
}
