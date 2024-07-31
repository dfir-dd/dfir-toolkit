use std::{collections::HashMap, path::Path};

use anyhow::bail;
use dfirtk_eventdata::SessionId;
use evtx::EvtxParser;
use walkdir::WalkDir;

use super::{Session, SessionEvent};

static KNOWN_FILES: &[&str] = &[
    "Security.evtx",
    "Microsoft-Windows-TerminalServices-RDPClient%4Operational.evtx",
    "Microsoft-Windows-TerminalServices-RemoteConnectionManager%4Operational.evtx",
    "Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx",
];

pub struct SessionStore {
    sessions: HashMap<SessionId, Session>,
}

impl SessionStore {
    pub fn import(evtx_files_dir: &Path, include_anonymous: bool) -> Result<Self, anyhow::Error> {
        if !evtx_files_dir.exists() {
            bail!(
                "directory '{}' does not exist. Aborting now.",
                evtx_files_dir.to_string_lossy()
            )
        }

        if !evtx_files_dir.is_dir() {
            bail!(
                "'{}' is no directory. Aborting now.",
                evtx_files_dir.to_string_lossy()
            );
        }

        let mut sessions = Self {
            sessions: HashMap::<SessionId, Session>::new(),
        };

        for filename in KNOWN_FILES {
            let mut path = evtx_files_dir.join(filename);

            // maybe we have troubles with case sensitivity
            // Let's try this:
            if !path.exists() {
                let mut files = WalkDir::new(evtx_files_dir)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|f| f.file_name().to_string_lossy().to_lowercase() == filename.to_lowercase());
                        
                if let Some(first_entry) = files.next() {
                    path = first_entry.into_path();

                    // there should be no more entry, otherwise
                    // the filename is unambigious
                    if let Some(next_entry) = files.next() {
                        log::error!(
                            "expected file '{filename}', but there exist \
                            multiple variants of this name. I found at least \
                            '{}' and '{}'. Omitting those files...",
                            path.file_name().unwrap().to_string_lossy(),
                            next_entry.file_name().to_string_lossy()
                        );
                        continue;
                    }
                }
            }

            if !path.is_file() {
                log::error!(
                    "tried to read '{}', but it is not a file. Omiting it...",
                    path.display()
                );
                continue;
            }

            log::info!("importing {} into session store", path.to_string_lossy());

            for event in EvtxParser::from_path(path)?
                .records_json_value()
                .map(|r| r.expect("error reading event"))
                .map(SessionEvent::try_from)
                .filter_map(|r| r.ok())
            {
                log::trace!("found session event at {}", event.record().timestamp);
                sessions.add_event(event);
            }
        }

        Ok(Self {
            sessions: sessions
                .sessions
                .into_iter()
                .filter(|s| {
                    if include_anonymous {
                        true
                    } else {
                        !s.1.is_anonymous()
                    }
                })
                .collect(),
        })
    }

    fn add_event(&mut self, event: SessionEvent) {
        if self.sessions.contains_key(event.session_id()) {
            self.sessions
                .entry(event.session_id().clone())
                .and_modify(|s| s.add_event(event));
        } else {
            self.sessions
                .insert(event.session_id().clone(), Session::from(event));
        }
    }

    pub fn find_session(&self, index: &str) -> Option<&Session> {
        self.sessions
            .iter()
            .find(|(k, _)| match k {
                SessionId::ActivityId(id)
                | SessionId::SessionName(id)
                | SessionId::LogonId(id)
                | SessionId::SessionId(id) => index == id,
                SessionId::None(id) => index == id.to_string(),
            })
            .map(|(_, v)| v)
    }
}

impl IntoIterator for SessionStore {
    type Item = Session;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = Vec::from_iter(self.sessions.into_values());
        v.sort();
        v.into_iter()
    }
}
