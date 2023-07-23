use std::fmt::Display;

use clap::ValueEnum;


#[derive(ValueEnum, Clone, Default)]
pub enum Protocol {
    Http,
    #[default] Https,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => write!(f, "http"),
            Protocol::Https => write!(f, "https"),
        }
    }
}