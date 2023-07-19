use std::{net::Ipv4Addr, str::FromStr, collections::HashMap};

use colored::{ColoredString, Colorize};
use lazy_regex::regex;
use regex::Regex;
use serde::Serialize;
use serde_json::{Value, json};

pub (crate) struct HighlightedStringBuilder {
    custom_regex: Option<Regex>
}

impl HighlightedStringBuilder {
    pub fn new(regex: Option<Regex>) -> Self {
        Self {
            custom_regex: regex
        }
    }

    pub fn highlight_data(&self, data: &Value) -> Value {
        match data {
            Value::String(s) => json!(self.build_from(&s[..])),
            Value::Array(o) => {
                let s: Vec<_> = o.iter().map(|d| self.highlight_data(d)).collect();
                json!(s)
            }
            Value::Object(o) => {
                let s: HashMap<_, _> = o.iter()
                    .map(|(k,v)| (
                        k, json!(self.highlight_data(v))
                    )).collect();
                json!(s)
            },
            Value::Null => json!(""),
            _ => data.clone(),
        }
    }

    pub fn build_from(&self, s: &str) -> HighlightedString {
        let ip_regex = regex!(r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b");
        let file_regex = regex!(r"[a-zA-Z]:\\\\?(?:[^\\]+\\\\?)+");
        if ip_regex.is_match(s) {
            for c in ip_regex.captures_iter(s) {
                for m in c.iter().flatten() {
                    let ip_addr = match Ipv4Addr::from_str(m.as_str()) {
                        Ok(addr) => addr,
                        Err(_) => {
                            log::warn!("invalid IP address: {}, don't highlighting it", m.as_str());
                            continue;
                        }
                    };
                    
                    if ip_addr.is_link_local() || ip_addr.is_loopback() || ip_addr.is_unspecified() {
                        continue;
                    }
                    if ip_addr.is_private() {
                        return s.bright_purple().into();
                    }
    
                    //if ip_addr.is_global() {
                        return s.red().on_bright_yellow().into();
                    //}
                }
            }
        }
    
        if file_regex.is_match(s) {
            return s.bright_green().into()
        }
    
        if s.to_lowercase().contains("admin") {
            return s.bright_yellow().on_red().into()
        }

        if let Some(regex) = self.custom_regex.as_ref() {
            if regex.is_match(s) {
                return s.blue().on_bright_white().into()
            }
        }
    
        s.bright_blue().into()
    }
}

pub (crate) struct HighlightedString {
    cstring: ColoredString
}

impl From<ColoredString> for HighlightedString {
    fn from(cstring: ColoredString) -> Self {
        Self {
            cstring
        }
    }
}

impl Serialize for HighlightedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let s = self.cstring.to_string();
        serializer.serialize_str(&s)
    }
}