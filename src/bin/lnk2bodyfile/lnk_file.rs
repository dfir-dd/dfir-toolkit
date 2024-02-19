use anyhow::bail;
use clio::Input;
use dfir_toolkit::common::bodyfile::Bodyfile3Line;
use encoding_rs::WINDOWS_1252;
use lnk::{LinkInfo, ShellLink, ShellLinkHeader};

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
        let localpath = self.lnk_file.link_target().unwrap_or("-".to_string());
        
        let atime = ShellLinkHeader::access_time(header);
        let mtime = ShellLinkHeader::write_time(header);
        let crtime = ShellLinkHeader::creation_time(header);

        let bfline = Bodyfile3Line::new()
            .with_name(&format!(
                "{} ({}, referred to by \"{}\")",
                localpath, self.lnk_file.string_data(), self.file_name,
            ))
            .with_size((*ShellLinkHeader::file_size(header)).into())
            .with_crtime(crtime.datetime().into())
            .with_mtime(mtime.datetime().into())
            .with_atime(atime.datetime().into());

        println!("{bfline}");
    }
}

impl TryFrom<&Input> for LnkFile {
    type Error = anyhow::Error;

    fn try_from(input: &Input) -> Result<Self, Self::Error> {
        let file_path = input.path().to_path_buf();
        let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        match ShellLink::open(file_path, WINDOWS_1252) {
            Ok(lnk_file) => Ok(Self {
                lnk_file,
                file_name,
            }),
            Err(e) => bail!(
                "{:?}: The file {} is not in a valid ShellLink format",
                e,
                file_name
            ),
        }
    }
}
