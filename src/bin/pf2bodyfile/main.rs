mod cli;
use cli::Cli;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use dfir_toolkit::scca::File;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_cli();

    if cli.prefetch_files().iter().any(|f| !f.can_seek()) {
        anyhow::bail!(
            "{} cannot read from a stream; you must specify a file",
            env!("CARGO_BIN_NAME")
        );
    }

    if cli.prefetch_files().iter().any(|f| ! f.path().is_file()) {
        anyhow::bail!(
            "{} you must specify a file",
            env!("CARGO_BIN_NAME")
        );
    }

    for input in cli.prefetch_files().iter() {
        let path = input.path().as_os_str().to_string_lossy();
        let pf_file = input.path().file_name().unwrap().to_string_lossy();
        let file = File::open(&path)?;
        let executable = file.utf8_executable_filename()?;
        let run_count = file.run_count()?;
        for time in file.last_run_times()? {
            let bf_line = Bodyfile3Line::new()
                .with_owned_name(format!("Prefetch: '{executable}' (run {run_count} times, read from '{pf_file}')"))
                .with_atime(time.into());
            println!("{bf_line}");
        }
    }
    Ok(())
}