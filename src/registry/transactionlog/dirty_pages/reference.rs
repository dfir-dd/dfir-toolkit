use binread::BinRead;
use derive_getters::Getters;

use crate::registry::Offset;

/// <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#new-format>
#[derive(BinRead, Debug, Clone, Default, Getters)]
pub struct DirtyPagesReference {
    /// Offset of a page in a primary file (in bytes),
    /// relative from the start of the hive bins data
    offset: Offset,

    /// Size of a page in bytes
    size: u32,
}

impl DirtyPagesReference {
    pub fn contains_offset(&self, offset: Offset) -> bool {
        self.offset.0 <= offset.0 && offset.0 < self.offset.0 + self.size
    }

    pub fn contains(&self, offset: Offset, size: usize) -> bool {
        if size == 0 {
            false
        } else {
            match TryInto::<u32>::try_into(size) {
                Ok(size) => {
                    let last_byte_offset = Offset(offset.0 + size - 1);
                    self.contains_offset(offset) && self.contains_offset(last_byte_offset)
                }
                Err(_) => false,
            }
        }
    }

    pub fn last_byte_offset(&self) -> Offset {
        Offset(self.offset.0 + self.size - 1)
    }
}

impl PartialOrd for DirtyPagesReference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.offset.partial_cmp(&other.offset) {
            Some(core::cmp::Ordering::Equal) => self.size.partial_cmp(&other.size),
            Some(std::cmp::Ordering::Less) => {
                if &self.last_byte_offset() >= other.offset() {
                    Some(std::cmp::Ordering::Less)
                } else {
                    // pages do overlap
                    None
                }
            }
            Some(std::cmp::Ordering::Greater) => {
                if self.offset() >= &other.last_byte_offset() {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    // pages do overlap
                    None
                }
            }
            None => None
        }
    }
}

impl Eq for DirtyPagesReference {

}

impl Ord for DirtyPagesReference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for DirtyPagesReference {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.size == other.size
    }
}

impl PartialEq<Offset> for DirtyPagesReference {
    fn eq(&self, offset: &Offset) -> bool {
        self.contains_offset(*offset)
    }
}

impl PartialOrd<Offset> for DirtyPagesReference {
    fn partial_cmp(&self, offset: &Offset) -> Option<std::cmp::Ordering> {
        if self.offset.0 > offset.0 {
            Some(std::cmp::Ordering::Greater)
        } else if self.contains_offset(*offset) {
            Some(std::cmp::Ordering::Equal)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}
