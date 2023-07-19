use std::{collections::HashMap, path::Path};

use eventdata::SessionId;
use evtx::EvtxParser;

use super::{Session, SessionEvent};

static KNOWN_FILES: & [&str] = &[
    "Security.evtx",
    "Microsoft-Windows-TerminalServices-RDPClient%4Operational.evtx",
    "Microsoft-Windows-TerminalServices-RemoteConnectionManager%4Operational.evtx",
    "Microsoft-Windows-TerminalServices-LocalSessionManager%4Operational.evtx"
];

pub struct SessionStore {
    sessions: HashMap<SessionId, Session>,
}

impl SessionStore {
    pub fn import(evtx_files_dir: &Path, include_anonymous: bool) -> Result<Self, anyhow::Error> {
        let mut sessions = Self {
            sessions: HashMap::<SessionId, Session>::new(),
        };


        for filename in KNOWN_FILES {
            let path = evtx_files_dir.join(filename);
            if !(path.exists() && path.is_file()) {
                log::warn!("unable to read file {}", path.display());
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
        self.sessions.iter().find(|(k, _)| {
            match k {
                SessionId::ActivityId(id)|
                SessionId::SessionName(id)|
                SessionId::LogonId(id) |
                SessionId::SessionId(id) => {
                    index == id
                }
                SessionId::None(id) => {
                    index == id.to_string()
                },
            }
        }).map(|(_, v)| v)
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