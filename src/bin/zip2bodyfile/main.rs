mod cli;

use std::io::{Read, Seek};

use chrono::FixedOffset;
use cli::Cli;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use dfir_toolkit::common::FancyParser;
use log::{error, warn};
use time::OffsetDateTime;
use zip::{ExtraField, ZipArchive};

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
    fn display_zip_file(
        &mut self,
        zip_file_name: &str,
        show_archive_name: bool,
    ) -> anyhow::Result<()>;
}

impl<R> DisplayZipFile for ZipArchive<R>
where
    R: Read + Seek,
{
    fn display_zip_file(
        &mut self,
        zip_file_name: &str,
        show_archive_name: bool,
    ) -> anyhow::Result<()> {
        for index in 0..self.len() {
            let file = self.by_index(index)?;

            let mut bf_line = Bodyfile3Line::new().with_size(file.size());

            let mut utc_mtime = None;
            for field in file.extra_data_fields() {
                #[allow(irrefutable_let_patterns)]
                if let ExtraField::ExtendedTimestamp(ts) = field {
                    if let Some(mtime) = ts.mod_time() {
                        bf_line = bf_line.with_mtime((mtime as i64).into());
                        utc_mtime = Some(mtime as i64);
                    }
                    if let Some(atime) = ts.mod_time() {
                        bf_line = bf_line.with_atime((atime as i64).into());
                    }
                    if let Some(crtime) = ts.mod_time() {
                        bf_line = bf_line.with_crtime((crtime as i64).into());
                    }
                    break;
                }
            }

            let tz_offset = utc_mtime.and_then(|utc_mtime| {
                file.last_modified().and_then(|last_modified| {
                    match OffsetDateTime::try_from(last_modified) {
                        Ok(local_ts) => {
                            let local_ts = local_ts.unix_timestamp();
                            match i32::try_from(local_ts - utc_mtime) {
                                Err(_) => {
                                    log::warn!("illegal timezone offset: {}, ", local_ts - utc_mtime);
                                    None
                                }
                                Ok(secs) => match FixedOffset::east_opt(secs) {
                                    None => {
                                        log::warn!("timestamp offset (abs value) is too large: {secs} seconds");
                                        None
                                    }
                                    Some(offset) => Some(offset),
                                },
                            }
                        }
                        Err(why) => {
                            log::warn!("unable to calculate timezone: {why}");
                            None
                        }
                    }
                })
            });

            let tz_offset_text = match tz_offset {
                None => "".to_string(),
                Some(o) => format!(", [offset: {o}]"),
            };

            if utc_mtime.is_none() {
                match file.last_modified() {
                    None => {
                        log::warn!("no extended timestamp header with modification time found");
                    }
                    Some(last_modified) => {
                        log::warn!("no extended timestamp header with modification time found, try using the MS-DOS timestamp instead");
                        match OffsetDateTime::try_from(last_modified) {
                            Err(why) => log::error!(
                                "unable to convert {last_modified} into an OffsetDateTime: {why}"
                            ),
                            Ok(ts) => bf_line = bf_line.with_mtime(ts.unix_timestamp().into()),
                        }
                    }
                }
            }

            let name = if show_archive_name {
                format!(
                    "{} (in archive {zip_file_name}){tz_offset_text}",
                    file.name()
                )
            } else {
                format!("{}{tz_offset_text}", file.name())
            };

            bf_line = bf_line.with_owned_name(name);

            println!("{bf_line}");
        }
        Ok(())
    }
}
