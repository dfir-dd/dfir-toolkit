//! This crates aims to be a replacement of <https://github.com/ColinFinck/nt-hive>, with the following
//! differences:
//! 
//!  - use of [BinRead](https://docs.rs/binread/latest/binread/) to parse hive files
//!  - support of displaying last written timestamps
//!  - possibly recovery of deleted cells (might be added in the future)
//! 
//! # Usage example
//! 
//! ```
//! # use std::error::Error;
//! use std::fs::File;
//! use nt_hive2::*;
//! 
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let hive_file = File::open("tests/data/testhive")?;
//! let mut hive = Hive::new(hive_file, HiveParseMode::NormalWithBaseBlock)?;
//! let root_key = hive.root_key_node()?;
//! 
//! for sk in root_key.subkeys(&mut hive)?.iter() {
//!     println!("\n[{}]; last written: {}", sk.borrow().name(), sk.borrow().timestamp());
//!     for value in sk.borrow().values() {
//!         println!("\"{}\" = {}", value.name(), value.value());
//!     }
//! }
//! # Ok(())
//! # }
//! ```
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod hive;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod hivebin;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod util;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod cell;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod nk;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod vk;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod db;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod subkeys_list;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod cell_with_u8_list;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
mod cell_iterator;

#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub mod transactionlog;

#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub use cell::*;
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub use cell_iterator::{CellIterator, CellLookAhead, CellSelector, CellFilter};
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub use hive::{Hive, Offset, HiveParseMode, ContainsHive, BaseBlock, CleanHive, DirtyHive, BASEBLOCK_SIZE, HiveWithLogs};
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub use nk::{KeyNode, KeyNodeWithMagic, SubPath};
#[cfg(any(feature = "regdump", feature = "hivescan", feature = "cleanhive"))]
pub use vk::{KeyValue, KeyValueWithMagic, RegistryValue};

pub mod types;