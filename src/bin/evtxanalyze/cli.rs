use std::{io::stdout, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use super::sessions::SessionStore;

#[derive(ValueEnum, Clone)]
pub enum Format {
    Json,
    Markdown,
    Csv,

    #[clap(name = "latex")]
    LaTeX,

    Dot,
}

#[derive(Subcommand)]
pub enum Command {
    /// generate a process tree
    #[clap(name = "pstree")]
    PsTree {
        /// display only processes of this user (case insensitive regex search)
        #[clap(short('U'), long("username"))]
        username: Option<String>,

        /// Name of the evtx file to parse
        evtx_file: PathBuf,

        /// output format
        #[clap(short('F'), long("format"), value_enum, default_value_t=Format::Csv)]
        format: Format,
    },

    /// display sessions
    #[clap(name = "sessions")]
    Sessions {
        /// Names of the evtx files to parse
        evtx_files_dir: PathBuf,

        /// include anonymous sessions
        #[clap(long("include-anonymous"))]
        include_anonymous: bool,
    },

    /// display one single session
    #[clap(name = "session")]
    Session {
        /// Names of the evtx files to parse
        evtx_files_dir: PathBuf,

        /// Session ID
        session_id: String,
    },
}

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub fn display_single_session(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Session {
                evtx_files_dir,
                session_id,
            } => {
                let sessions = SessionStore::import(evtx_files_dir, true)?;
                match sessions.find_session(session_id) {
                    None => log::error!("no value found for session id {session_id}"),
                    Some(session) => {
                        let mut csv_writer = csv::Writer::from_writer(stdout());
                        for event in session.iter_events() {
                            event.to_csv(&mut csv_writer)?;
                        }
                        csv_writer.flush()?;
                    }
                }
                Ok(())
            }

            _ => unreachable!(),
        }
    }
    pub fn display_sessions(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Sessions {
                evtx_files_dir,
                include_anonymous,
            } => {
                let sessions = SessionStore::import(evtx_files_dir, *include_anonymous)?;

                let mut csv_writer = csv::Writer::from_writer(stdout());
                for session in sessions {
                    session.into_csv(&mut csv_writer)?;
                }
                csv_writer.flush()?;
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
