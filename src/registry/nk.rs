use std::cell::Ref;
use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::rc::Rc;

use super::Cell;
use super::Hive;
use super::hive::CleanHive;
use super::subkeys_list::*;
use super::Offset;
use super::vk::KeyValueCell;
use super::vk::KeyValueList;
use super::vk::KeyValue;
use super::vk::KeyValueWithMagic;
use binread::BinRead;
use binread::BinResult;
use binread::FilePtr32;
use binread::ReadOptions;
use binread::derive_binread;
use binread::BinReaderExt;
use bitflags::bitflags;
use chrono::DateTime;
use chrono::Utc;
use super::util::{parse_string, parse_timestamp};


#[derive(BinRead)]
#[br(magic = b"nk")]
pub struct KeyNodeWithMagic(KeyNode);

/// represents a registry key node (as documented in <https://github.com/msuhanov/regf/blob/master/Windows%20registry%20file%20format%20specification.md#key-node>)
#[allow(dead_code)]
#[derive_binread]
pub struct KeyNode {
    #[br(parse_with=parse_node_flags)]
    pub (crate) flags: KeyNodeFlags,
    
    #[br(parse_with=parse_timestamp)]
    timestamp: DateTime<Utc>,
    access_bits: u32,
    pub parent: Offset,
    subkey_count: u32,

    #[br(temp)]
    volatile_subkey_count: u32,
    subkeys_list_offset: Offset,

    #[br(temp)]
    volatile_subkeys_list_offset: Offset,

    #[br(temp)]
    key_values_count: u32,

    #[br(   if(key_values_count > 0),
            deref_now,
            restore_position,
            args(key_values_count as usize))]
    key_values_list: Option<FilePtr32<Cell<KeyValueList, (usize,)>>>,

    #[br(temp)]
    key_values_list_offset: u32,

    #[br(temp)]
    key_security_offset: Offset,
    
    #[br(temp)]
    class_name_offset: Offset,

    #[br(temp)]
    max_subkey_name: u32,

    #[br(temp)]
    max_subkey_class_name: u32,

    #[br(temp)]
    max_value_name: u32,

    #[br(temp)]
    max_value_data: u32,

    #[br(temp)]
    work_var: u32,

    #[br(temp)]
    key_name_length: u16,

    #[br(temp)]
    class_name_length: u16,

    #[br(   parse_with=parse_string,
            count=key_name_length,
            args(flags.contains(KeyNodeFlags::KEY_COMP_NAME)))]
    key_name_string: String,

    #[br(   if(key_values_count > 0 && key_values_list_offset != u32::MAX),
            parse_with=read_values,
            args(key_values_list.as_ref(), ))]
    values: Vec<KeyValue>,

    #[br(default)]
    subkeys: Rc<RefCell<Vec<Rc<RefCell<Self>>>>>
}

fn parse_node_flags<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ())
-> BinResult<KeyNodeFlags>
{
    let raw_value: u16 = reader.read_le()?;
    Ok(KeyNodeFlags::from_bits(raw_value).unwrap())
}

bitflags! {
    pub(crate) struct KeyNodeFlags: u16 {
        /// This is a volatile key (not stored on disk).
        const KEY_IS_VOLATILE = 0x0001;
        /// This is the mount point of another hive (not stored on disk).
        const KEY_HIVE_EXIT = 0x0002;
        /// This is the root key.
        const KEY_HIVE_ENTRY = 0x0004;
        /// This key cannot be deleted.
        const KEY_NO_DELETE = 0x0008;
        /// This key is a symbolic link.
        const KEY_SYM_LINK = 0x0010;
        /// The key name is in (extended) ASCII instead of UTF-16LE.
        const KEY_COMP_NAME = 0x0020;
        /// This key is a predefined handle.
        const KEY_PREDEF_HANDLE = 0x0040;
        /// This key was virtualized at least once.
        const KEY_VIRT_MIRRORED = 0x0080;
        /// This is a virtual key.
        const KEY_VIRT_TARGET = 0x0100;
        /// This key is part of a virtual store path.
        const KEY_VIRTUAL_STORE = 0x0200;
    }
}

impl KeyNode
{
    /// Returns the name of this Key Node.
    pub fn name(&self) -> &str {
        &self.key_name_string
    }

    /// Returns the time when this node has been written last.
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Returns the number of subkeys
    pub fn subkey_count(&self) -> u32 {
        self.subkey_count
    }

    /// Returns a list of subkeys.
    /// 
    /// This function caches the subkeys, so the first call to this function might be slower.
    pub fn subkeys<B>(&self, hive: &mut Hive<B, CleanHive>) -> BinResult<Ref<Vec<Rc<RefCell<Self>>>>> where B: BinReaderExt {
        if self.subkeys.borrow().is_empty() && self.subkey_count() > 0 {
            let sk = self.read_subkeys(hive)?;
            *self.subkeys.borrow_mut() = sk;
        }
        Ok(self.subkeys.borrow())
    }

    fn read_subkeys<B>(&self, hive: &mut Hive<B, CleanHive>) -> BinResult<Vec<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let offset = self.subkeys_list_offset;

        if offset.0 == u32::MAX{
            return Ok(Vec::new());
        }

        let subkeys_list: SubKeysList = hive.read_structure(offset)?;

        log::debug!("SubKeyList is of type '{}'", match subkeys_list {
            SubKeysList::IndexLeaf { items: _, ..} => "IndexLeaf",
            SubKeysList::FastLeaf { items: _, ..} => "FastLeaf",
            SubKeysList::HashLeaf { items: _, ..} => "HashLeaf",
            SubKeysList::IndexRoot { items: _, ..} => "IndexRoot",
        });

        log::trace!("{:?}", subkeys_list);

        if subkeys_list.is_index_root() {
            log::debug!("reading indirect subkey lists");
            let subkeys: BinResult<Vec<_>>= subkeys_list.into_offsets().map(|o| {
                let subsubkeys_list: SubKeysList = hive.read_structure(o)?;
                assert!(!subsubkeys_list.is_index_root());

                let subkeys: BinResult<Vec<_>> = subsubkeys_list.into_offsets().map(|o2| {
                    let nk: KeyNodeWithMagic = hive.read_structure(o2)?;
                    Ok(Rc::new(RefCell::new(nk.0)))
                }).collect();
                subkeys
            }).collect();

            match subkeys {
                Err(why) => Err(why),
                Ok(sk) => Ok(sk.into_iter().flatten().collect())
            }
        } else {
            log::debug!("reading single subkey list");
            let subkeys: BinResult<Vec<_>> = subkeys_list.into_offsets().map(|offset| {
                let nk: KeyNodeWithMagic = hive.read_structure(offset)?;
                Ok(Rc::new(RefCell::new(nk.0)))
            }).collect();
            subkeys
        }
    }
    

    fn subpath_parts<B>(&self, mut path_parts: Vec<&str>, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        if let Some(first) = path_parts.pop() {
            if let Some(top) = self.subkey(first, hive)? {
                return if path_parts.is_empty() {
                    Ok(Some(top))
                } else {
                    top.borrow().subpath_parts(path_parts, hive)
                };
            }
        }
        Ok(None)
    }

    /// returns the subkey with a given `name`, or [`None`] if there is no such subkey.
    /// The name is compared without case sensitivity, because
    /// 
    /// > Each key has a name consisting of one or more printable characters.
    /// > *Key names are not case sensitive.* Key names cannot include the backslash character (\),
    /// > but any other printable character can be used. Value names and data can include the backslash character.
    /// 
    /// (<https://learn.microsoft.com/en-us/windows/win32/sysinfo/structure-of-the-registry>)
    pub fn subkey<B>(&self, name: &str, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let lowercase_name = name.to_lowercase();
        let subkey = self.subkeys(hive)?
            .iter()
            .find(|s|s.borrow().name().to_lowercase() == lowercase_name)
            .map(Rc::clone);
        Ok(subkey)
    }

    /// returns the list of all [KeyValue]s of this key
    pub fn values(&self) -> &Vec<KeyValue> {
        &self.values
    }
}

pub trait SubPath<T> {
    fn subpath<B>(&self, path: T, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt;
}

impl SubPath<&str> for KeyNode {
    fn subpath<B>(&self, path: &str, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let path_parts: Vec<_> = path.split('\\').rev().collect();
        self.subpath_parts(path_parts, hive)
    }
}

impl SubPath<&String> for KeyNode {
    fn subpath<B>(&self, path: &String, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let path_parts: Vec<_> = path.split('\\').rev().collect();
        self.subpath_parts(path_parts, hive)
    }
}

impl SubPath<&Vec<&str>> for KeyNode {
    fn subpath<B>(&self, path: &Vec<&str>, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let path_parts: Vec<_> = path.iter().rev().copied().collect();
        self.subpath_parts(path_parts, hive)
    }
}

impl SubPath<&Vec<String>> for KeyNode {
    fn subpath<B>(&self, path: &Vec<String>, hive: &mut Hive<B, CleanHive>) -> BinResult<Option<Rc<RefCell<Self>>>> where B: BinReaderExt {
        let path_parts: Vec<_> = path.iter().rev().map(|s| &s[..]).collect();
        self.subpath_parts(path_parts, hive)
    }
}

fn read_values<R: Read + Seek>(
    reader: &mut R,
    _ro: &ReadOptions,
    args: (Option<&FilePtr32<KeyValueCell>>, ),
) -> BinResult<Vec<KeyValue>> {
    Ok(match args.0 {
        None => Vec::new(),
        Some(key_values_list) => match &key_values_list.value {
            None => Vec::new(),
            Some(kv_list_cell) => {
                let kv_list: &KeyValueList = kv_list_cell.data();
                let mut result = Vec::with_capacity(kv_list.key_value_offsets.len());
                for offset in kv_list.key_value_offsets.iter() {
                    reader.seek(SeekFrom::Start(offset.0.into()))?;
                    let vk_result: BinResult<Cell<KeyValueWithMagic, ()>> = reader.read_le();
                    match vk_result {
                        Ok(vk) => result.push(vk.into()),
                        Err(why) => {
                            log::debug!("error while parsing KeyValue: {}", why);
                        }
                    }
                }
                result
            }
        }
    })
}

impl From<Cell<KeyNodeWithMagic, ()>> for KeyNodeWithMagic {
    fn from(cell: Cell<KeyNodeWithMagic, ()>) -> Self {
        cell.into_data()
    }
}

impl From<KeyNodeWithMagic> for KeyNode {
    fn from(mkn: KeyNodeWithMagic) -> Self {
        mkn.0
    }
}