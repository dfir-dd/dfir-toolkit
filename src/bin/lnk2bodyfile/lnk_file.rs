use anyhow::bail;
use chrono::{DateTime, Utc};
use clio::Input;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use lnk::{ShellLinkHeader, ShellLink, LinkInfo};


pub struct LnkFile {
    lnk_file: ShellLink,
    file_name: String,
}

impl LnkFile {
    pub fn print_bodyfile(&self) {
        self.print_bodyfile_for_me();
    }

    fn print_bodyfile_for_me(&self) {
        let header = self.lnk_file.header();
        let localpath = match self.lnk_file.link_info() {
            Some(s1) => match LinkInfo::local_base_path(s1) {
                Some(s2) => s2,
                None => "-",
            },
            None => "-",
        };
        let arguments = match self.lnk_file.arguments() {
            Some(s) => s,
            None => "-",
        };
        let atime = ShellLinkHeader::access_time(header);
        let mtime = ShellLinkHeader::write_time(header);
        let ctime = ShellLinkHeader::creation_time(header);

        let bfline = Bodyfile3Line::new()
            .with_name(&format!("{} {} (referred to by \"{}\")", localpath, arguments, self.file_name))
            .with_size(ShellLinkHeader::file_size(header).into())
            .with_ctime(DateTime::<Utc>::from_naive_utc_and_offset(ctime.datetime(), Utc).timestamp())
            .with_mtime(DateTime::<Utc>::from_naive_utc_and_offset(mtime.datetime(), Utc).timestamp())
            .with_atime(DateTime::<Utc>::from_naive_utc_and_offset(atime.datetime(), Utc).timestamp());

        println!("{bfline}");
    }
}

impl TryFrom<&Input> for LnkFile {
    type Error = anyhow::Error;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let file_path = input.path().to_path_buf();
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        match ShellLink::open(file_path) {
            Ok(lnk_file) => Ok ( Self { lnk_file, file_name }),
            Err(e) => bail!("{:?}: The file {} is not in a valid ShellLink format", e, file_name),
        }
    }
}

