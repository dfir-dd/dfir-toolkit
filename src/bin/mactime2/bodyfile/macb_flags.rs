use std::fmt;

use bitflags::bitflags;
use serde::Serialize;

bitflags! {
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct MACBFlags: u8 {
        const NONE = 0b00000000;
        const M = 0b00000001;
        const A = 0b00000010;
        const C = 0b00000100;
        const B = 0b00001000;
    }
}

impl fmt::Display for MACBFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = if *self & Self::M == Self::M { 'm' } else { '.' };
        let a = if *self & Self::A == Self::A { 'a' } else { '.' };
        let c = if *self & Self::C == Self::C { 'c' } else { '.' };
        let b = if *self & Self::B == Self::B { 'b' } else { '.' };
        write!(f, "{}{}{}{}", m, a, c, b)
    }
}

impl Serialize for MACBFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}
