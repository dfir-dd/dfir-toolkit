use binread::{derive_binread, BinRead};
use std::any::Any;

#[allow(unused_imports)]
use crate::*;

/// Represents the header of a [Cell]. Technically, a cell header only contains
/// the size of the cell as a 32bit value, but [CellHeader] enriches this by
/// some additional information
#[derive_binread]
#[derive(Eq, PartialEq)]
pub struct CellHeader {
    // The cell size must be a multiple of 8 bytes
    #[br(temp, assert(raw_size != 0))]
    raw_size: i32,

    #[br(calc((raw_size as i64).abs().try_into().unwrap()))]
    size: usize,

    #[br(calc(raw_size > 0))]
    is_deleted: bool,
}

impl CellHeader {
    /// Returns the size of the header.
    ///
    /// This is *not* the stored size value, but the *absolute* value of it.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the size of the [Cell] content, which equals to the size of the cell
    /// minus the size of its header
    pub fn contents_size(&self) -> usize {
        assert!(self.size() >= 4);
        self.size() - std::mem::size_of::<i32>()
    }

    /// returns [true] iff the [Cell] is considered as being *deleted*
    ///
    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }
}

/// A [Cell] represents the most basic data structure of hive files.
/// Nearly every other data is stored as content of a [Cell].
///
/// As [Cell] is a generic, it receives two generic arguments:
///  - `T` denotes the type contained in the [Cell]
///  - `A` specifies the arguments required by [binread] to correctly parse an object of type `T`
///
/// # Usage
/// If you know what kind of data should be stored in a certain [Cell],
/// you can simply read it. Assume you have [Cell] which should contain
/// a [`KeyNode`](struct@KeyNode), you can read it as follows:
///
/// ```
/// # use std::error::Error;
/// # use std::fs::File;
/// use nt_hive2::*;
/// use std::io::{Seek, SeekFrom};
/// use binread::BinReaderExt;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// # let hive_file = File::open("tests/data/testhive")?;
/// # let mut hive = Hive::new(hive_file, HiveParseMode::NormalWithBaseBlock)?;
/// # let offset = hive.root_cell_offset();
/// hive.seek(SeekFrom::Start(offset.0.into()))?;
/// let cell: Cell<KeyNodeWithMagic, ()> = hive.read_le().unwrap();
/// let my_node: KeyNode = {
///     let knwm: KeyNodeWithMagic = cell.into();
///     knwm.into()
/// };
/// # Ok(())
/// # }
/// ```
///
/// For conveniance reasons, [Hive] already presents the method [read_structure](Hive::read_structure),
/// which does basically the same.
///
#[derive(BinRead, Eq, PartialEq)]
#[br(import_tuple(data_args: A))]
pub struct Cell<T, A: Any + Copy>
where
    T: BinRead<Args = A>,
{
    header: CellHeader,

    #[br(args_tuple(data_args))]
    data: T,
}

impl<T, A> Cell<T, A>
where
    T: BinRead<Args = A>,
    A: Any + Copy,
{
    /// returns [true] iff the [Cell] is considered as being *deleted*
    ///
    pub fn is_deleted(&self) -> bool {
        self.header.is_deleted
    }

    /// returns [true] iff the [Cell] is considered as being *allocated*.
    /// This is a conveniance function which simply calls [is_deleted](Self::is_deleted)
    /// and negates the result.
    ///
    pub fn is_allocated(&self) -> bool {
        !self.is_deleted()
    }

    /// returns a reference to the contained data structure
    pub fn data(&self) -> &T {
        &self.data
    }

    /// consumes the [Cell] and returns the contained data structure   
    pub(crate) fn into_data(self) -> T {
        self.data
    }
}
