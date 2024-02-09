mod cli;
use std::path::Path;

use cli::Cli;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use forensic_rs::prelude::*;
use frnsc_prefetch::prelude::*;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_cli();

    if cli.prefetch_files().iter().any(|f| !f.path().is_file()) {
        anyhow::bail!("{} you must specify a file", env!("CARGO_BIN_NAME"));
    }
    let mut fs = ChRootFileSystem::new(Path::new("."), Box::new(StdVirtualFS::new()));

    for input in cli.prefetch_files().iter() {

        let pf_file_name = input.path().file_name().unwrap().to_string_lossy();
        let pf_file = read_prefetch_file(&pf_file_name, fs.open(input.path())?)?;

        for time in pf_file.last_run_times {
            let accessed =
                winstructs::timestamp::WinTimestamp::new(&time.filetime().to_le_bytes())?
                    .to_datetime()
                    .into();

            let bf_line = Bodyfile3Line::new()
                .with_owned_name(format!(
                    "Prefetch: run '{}' (run {} times, read from '{pf_file_name}')",
                    pf_file.name, pf_file.run_count
                ))
                .with_atime(accessed);
            println!("{bf_line}");

            if *cli.include_metrics() {
                for metric in &pf_file.metrics {
                    let mf = &metric.file;
                    let bf_line = Bodyfile3Line::new()
                        .with_owned_name(format!(
                            "Prefetch: running '{} possibly loaded '{mf}', read from '{pf_file_name}')",
                            pf_file.name
                        ))
                        .with_atime(accessed);
                    println!("{bf_line}");
                }
            }
        }
    }
    Ok(())
}

