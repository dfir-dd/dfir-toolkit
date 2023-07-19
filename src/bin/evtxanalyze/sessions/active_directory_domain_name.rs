use std::fmt::Display;
use std::hash::Hash;

use serde::Serialize;

#[derive(Debug, Clone, Eq, Serialize)]
pub struct ActiveDirectoryDomainName(String);

impl From<String> for ActiveDirectoryDomainName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ActiveDirectoryDomainName {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Display for ActiveDirectoryDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for ActiveDirectoryDomainName {
    fn eq(&self, other: &Self) -> bool {
        let me: String = self.0.to_lowercase();
        let other = other.0.to_lowercase();

        if me.len() > 15 && other.len() == 15 {
            me[..15] == other
        } else if me.len() == 15 && other.len() > 15 {
            me == other[..15]
        } else {
            me == other
        }
    }
}

impl Hash for ActiveDirectoryDomainName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}