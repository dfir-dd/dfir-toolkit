use std::{fmt::Display, str::FromStr};
use chrono_tz::TZ_VARIANTS;
use chrono_tz::Tz;

#[derive(Clone, Debug, Copy)]
pub enum TzArgument {
    List,
    Tz(Tz),
}

impl FromStr for TzArgument {
    type Err = chrono_tz::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "list" {
            Ok(Self::List)
        } else {
            Ok(Self::Tz(s.parse()?))
        }
    }
}

impl Display for TzArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TzArgument::List => write!(f, "list"),
            TzArgument::Tz(tz) => tz.fmt(f),
        }
    }
}


impl TzArgument {
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List)
    }
    pub fn is_tz(&self) -> bool {
        matches!(self, Self::Tz(_))
    }
    pub fn into_tz(self) -> Option<Tz> {
        match self {
            TzArgument::List => None,
            TzArgument::Tz(tz) => Some(tz),
        }
    }
    pub fn display_zones() {
        for v in TZ_VARIANTS.iter() {
            println!("{}", v);
        }
    }
}