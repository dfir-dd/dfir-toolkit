use std::{
    collections::{BTreeSet, HashSet},
    io::Write,
};

use super::{ActiveDirectoryDomainName, SessionAsCsv, SessionAsJson, SessionEvent};
use eventdata::SessionId;

pub struct Session {
    events: BTreeSet<SessionEvent>,
    session_id: SessionId,
    domain: Option<ActiveDirectoryDomainName>,
    usernames: HashSet<String>,
    clients: HashSet<String>,
    server: Option<String>,
    computer: String,
}

impl Session {
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn iter_events(&self) -> impl Iterator<Item = &SessionEvent> {
        self.events.iter()
    }

    pub fn add_event(&mut self, event: SessionEvent) {
        assert_eq!(event.session_id(), &self.session_id);
        let mut domain_from_username = None;
        let username;

        if let Some(u) = event.event_type().username(event.record()) {
            if u.contains('\\') {
                let mut parts: Vec<_> = u.split('\\').map(|s| s.to_owned()).collect();
                assert_eq!(parts.len(), 2);
                username = parts.pop().unwrap();
                domain_from_username = Some(parts.pop().unwrap());
            } else {
                username = u;
            }
            self.usernames.insert(username);
        }

        if let Some(addr) = event.event_type().client_address(event.record()) {
            if let Some(hostname) = event.event_type().client_hostname(event.record()) {
                self.clients.insert(format!("{hostname}({addr})"));
            } else {
                self.clients.insert(addr);
            }
        } else if let Some(hostname) = event.event_type().client_hostname(event.record()) {
            self.clients.insert(hostname);
        }

        let server = if let Some(addr) = event.event_type().server_address(event.record()) {
            if let Some(hostname) = event.event_type().server_hostname(event.record()) {
                Some(format!("{hostname}({addr})"))
            } else {
                Some(addr)
            }
        } else {
            event.event_type().server_hostname(event.record())
        };

        if let Some(server) = server {
            match &self.server {
                None => self.server = Some(server),
                Some(s) => {
                    assert_eq!(s, &server, "multiple servers on one single connection are not supported: {s} != {server}");
                }
            }
        }

        if let Some(domain_from_record) = event.event_type().domain(event.record()) {
            let domain = match domain_from_username {
                Some(domain) if !domain.is_empty() => {
                    assert_eq!(domain, domain_from_record, "multiple domains on one single connection are not supported: {domain} != {domain_from_record}");
                    Some(ActiveDirectoryDomainName::from(domain))
                }
                _ if ! domain_from_record.is_empty() => Some(ActiveDirectoryDomainName::from(domain_from_record)),
                _ => None
            };

            match &self.domain {
                None => self.domain = domain,
                Some(d) => if let Some(new_domain) = domain {
                    if d != &new_domain {
                        log::warn!("multiple domains on one single connection are not supported: {d} != {new_domain}, failed event was {event}", event = event.record().data);
                    }
                }
            }
        }

        self.events.insert(event);
    }

    pub fn first_event(&self) -> &SessionEvent {
        debug_assert!(!self.events.is_empty());
        self.events.first().unwrap()
    }

    pub fn last_event(&self) -> &SessionEvent {
        debug_assert!(!self.events.is_empty());
        self.events.last().unwrap()
    }

    pub fn into_csv<W>(self, writer: &mut csv::Writer<W>) -> csv::Result<()>
    where
        W: Write,
    {
        writer.serialize(&Into::<SessionAsCsv>::into(self))
    }

    pub fn is_anonymous(&self) -> bool {
        if self.usernames.is_empty() {
            false
        } else {
            !self.usernames.iter().any(|u| !u.starts_with("ANONYMOUS"))
        }
    }
}

impl From<SessionEvent> for Session {
    fn from(value: SessionEvent) -> Self {
        log::trace!(
            "creating new session, starting at {}",
            value.record().timestamp
        );

        let events = BTreeSet::<SessionEvent>::new();
        let session_id = (*value.session_id()).clone();

        let computer = value.record().data["Event"]["System"]["Computer"]
            .as_str()
            .unwrap()
            .to_owned();

        let mut me = Self {
            events,
            session_id,
            domain: None,
            usernames: HashSet::new(),
            clients: HashSet::new(),
            server: None,
            computer,
        };

        me.add_event(value);
        me
    }
}

impl Ord for Session {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first_event().cmp(other.first_event())
    }
}

impl PartialOrd for Session {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Session {}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.first_event().eq(other.first_event())
    }
}

#[allow(clippy::from_over_into)]
impl Into<SessionAsJson> for Session {
    fn into(self) -> SessionAsJson {
        let begin = self.first_event().record().timestamp;
        let end = self.last_event().record().timestamp;
        let duration = end - begin;
        let session_id = self.session_id().clone();
        let events = self.events.len();
        SessionAsJson {
            begin,
            end,
            duration,
            session_id,
            usernames: self.usernames,
            events,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<SessionAsCsv> for Session {
    fn into(self) -> SessionAsCsv {
        let begin = self.first_event().record().timestamp;
        let end = self.last_event().record().timestamp;
        let duration = end - begin;
        let session_id = self.session_id().clone();
        let events = self.events.len();
        let usernames: Vec<_> = self.usernames.into_iter().collect();
        let clients: Vec<_> = self.clients.into_iter().collect();
        SessionAsCsv {
            begin,
            end,
            duration,
            session_id,
            domain: self.domain.clone(),
            usernames: usernames.join(", "),
            clients: clients.join(", "),
            server: self.server,
            computer: self.computer,
            events,
        }
    }
}
