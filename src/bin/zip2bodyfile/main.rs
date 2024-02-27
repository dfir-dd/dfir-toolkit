mod cli;

use std::io::{Read, Seek};

use cli::Cli;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use log::{error, warn};
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse_cli();

    if cli.zip_files().iter().any(|f| !f.path().exists()) {
        anyhow::bail!("some files you specified do not exist");
    }
    if cli.zip_files().iter().any(|f| !f.path().is_file()) {
        anyhow::bail!("some paths you specified are no files");
    }
    for input in cli.zip_files().iter() {
        if let Some(zip_os_filename) = input.path().file_name() {
            if let Some(zip_filename) = zip_os_filename.to_str() {
                let mut zip_archive = ZipArchive::new(input.clone().open()?)?;
                zip_archive.display_zip_file(zip_filename, *cli.show_archive_name())?;
            } else {
                error!("invalid Unicode characters in filename: '{zip_os_filename:?}'")
            }
        } else {
            warn!("unable to handle directories; you must specify concrete file names");
        }
    }
    Ok(())
}

trait DisplayZipFile {
    fn display_zip_file(&mut self, zip_file_name: &str, show_archive_name: bool) -> anyhow::Result<()>;
}

impl<R> DisplayZipFile for ZipArchive<R> where R: Read + Seek {
    fn display_zip_file(&mut self, zip_file_name: &str, show_archive_name: bool) -> anyhow::Result<()> {
        for index in 0..self.len() {
            let file = self.by_index(index)?;

            let name = if show_archive_name {
                format!("{} (in archive {zip_file_name})", file.name())
            } else {
                file.name().to_string()
            };

            let bf_line = Bodyfile3Line::new()
                .with_owned_name(name)
                .with_size(file.size())
                .with_mtime(file.last_modified().to_time()?.unix_timestamp().into());

            println!("{bf_line}");
        }
        Ok(())
    }
}
