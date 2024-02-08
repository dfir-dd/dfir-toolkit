mod cli;
use std::path::Path;

use anyhow::bail;
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
        let _path = input.path().as_os_str().to_string_lossy();
        let pf_file_name = input.path().file_name().unwrap().to_string_lossy();
        let file = fs.open(input.path()).or_else(|why| bail!("{why}"))?;
        let pf_file = read_prefetch_file(&pf_file_name, file).or_else(|why| bail!("{why}"))?;
        let executable = &pf_file.name;
        let run_count = &pf_file.run_count;

        for time in pf_file.last_run_times {
            let ts = winstructs::timestamp::WinTimestamp::new(&time.filetime().to_le_bytes())?;
            let accessed = ts.to_datetime().into();
            let bf_line = Bodyfile3Line::new()
                .with_owned_name(format!(
                    "Prefetch: run '{executable}' (run {run_count} times, read from '{pf_file_name}')"
                ))
                .with_atime(accessed);
            println!("{bf_line}");

            if *cli.include_metrics() {
                for metric in &pf_file.metrics {
                    let mf = &metric.file;
                    let bf_line = Bodyfile3Line::new()
                        .with_owned_name(format!(
                            "Prefetch: running '{executable} loads '{mf}', read from '{pf_file_name}')"
                        ))
                        .with_atime(accessed);
                    println!("{bf_line}");
                }
            }
        }
    }
    Ok(())
}
