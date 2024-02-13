use crate::common::bodyfile::Bodyfile3ParserError;
use std::fmt::Display;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Accessed(Option<i64>);
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Modified(Option<i64>);
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Changed(Option<i64>);
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Created(Option<i64>);

pub trait BehavesLikeI64: From<i64> + From<Option<i64>> {
    fn as_ref(&self) -> Option<&i64>;
    fn is_none(&self) -> bool;
    fn is_some(&self) -> bool;
}

macro_rules! behaves_like_i64 {
    ($t: ty, $error: expr) => {
        impl BehavesLikeI64 for $t {
            fn as_ref(&self) -> Option<&i64> {
                self.0.as_ref()
            }
            fn is_none(&self) -> bool {
                self.0.is_none()
            }
            fn is_some(&self) -> bool {
                self.0.is_some()
            }
        }

        impl From<i64> for $t {
            fn from(v: i64) -> Self {
                if v == -1 {
                    Self(None)
                } else {
                    Self(Some(v))
                }
            }
        }

        impl From<Option<i64>> for $t {
            fn from(v: Option<i64>) -> Self {
                Self(v)
            }
        }

        impl From<NaiveDateTime> for $t {
            fn from(v: NaiveDateTime) -> Self {
                Self(Some(DateTime::<Utc>::from_naive_utc_and_offset(v, Utc).timestamp()))
            }
        }

        impl From<&NaiveDateTime> for $t {
            fn from(v: &NaiveDateTime) -> Self {
                Self(Some(DateTime::<Utc>::from_naive_utc_and_offset(*v, Utc).timestamp()))
            }
        }

        impl From<DateTime::<Utc>> for $t {
            fn from(v: DateTime::<Utc>) -> Self {
                Self(Some(v.timestamp()))
            }
        }

        impl From<&DateTime::<Utc>> for $t {
            fn from(v: &DateTime::<Utc>) -> Self {
                Self(Some(v.timestamp()))
            }
        }

        impl TryFrom<&str> for $t {
            type Error = Bodyfile3ParserError;
            fn try_from(
                val: &str,
            ) -> std::result::Result<Self, <Self as std::convert::TryFrom<&str>>::Error> {
                let my_time = str::parse::<i64>(val).or(Err($error))?;
                if my_time < -1 {
                    Err($error)
                } else if my_time == -1 {
                    Ok(Self::default())
                } else {
                    Ok(Self(Some(my_time)))
                }
            }
        }

        impl Display for $t {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                match self.0 {
                    None => write!(f, "-1"),
                    Some(v) => write!(f, "{v}"),
                }
            }
        }
    };
}

behaves_like_i64!(Accessed, Bodyfile3ParserError::IllegalATime);
behaves_like_i64!(Modified, Bodyfile3ParserError::IllegalMTime);
behaves_like_i64!(Changed, Bodyfile3ParserError::IllegalCTime);
behaves_like_i64!(Created, Bodyfile3ParserError::IllegalCRTime);
