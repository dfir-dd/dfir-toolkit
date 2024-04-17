use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
    rc::Rc,
};

use binread::BinReaderExt;
use nt_hive2::{CleanHive, Hive, KeyNode, Offset};

use crate::regtreeentry::*;

pub(crate) struct RegTreeBuilder {
    /// contains all RegTreeEntrys with no parent
    subtrees: HashMap<Offset, Rc<RefCell<RegTreeEntry>>>,

    /// contains all (really all) RegTreeEntrys
    entries: HashMap<Offset, Rc<RefCell<RegTreeEntry>>>,

    /// contains the offsets of all non-added entries which are parents of
    /// already added entries, and the entries that miss their parents
    missing_parents: HashMap<Offset, Vec<Rc<RefCell<RegTreeEntry>>>>,
}

impl<B> From<Hive<B, CleanHive>> for RegTreeBuilder
where
    B: BinReaderExt,
{
    fn from(hive: Hive<B, CleanHive>) -> Self {
        Self::from_hive(hive, |_| ())
    }
}

pub(crate) struct RootNodes<'a> {
    values: hash_map::Values<'a, Offset, Rc<RefCell<RegTreeEntry>>>,
}

impl<'a> Iterator for RootNodes<'a> {
    type Item = &'a Rc<RefCell<RegTreeEntry>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next()
    }
}

impl RegTreeBuilder {
    pub fn from_hive<B, C>(hive: Hive<B, CleanHive>, progress_callback: C) -> Self
    where
        B: BinReaderExt,
        C: Fn(u64),
    {
        let mut me = Self {
            subtrees: HashMap::new(),
            entries: HashMap::new(),
            missing_parents: HashMap::new(),
        };

        let mut last_offset = Offset(0);
        for cell in hive.hivebins().flat_map(|hivebin| hivebin.cells()) {
            let my_offset = *cell.offset();
            let is_deleted = cell.header().is_deleted();
            assert_ne!(last_offset, my_offset);
            log::trace!("found new cell at offset 0x{:x}", my_offset.0);

            if let Ok(nk) = TryInto::<KeyNode>::try_into(cell) {
                me.insert_nk(my_offset, nk, is_deleted);
            }
            last_offset = my_offset;
            progress_callback(last_offset.0.into());
        }
        me
    }

    pub fn root_nodes(&self) -> RootNodes {
        RootNodes {
            values: self.subtrees.values(),
        }
    }

    fn insert_nk(&mut self, nk_offset: Offset, nk: KeyNode, is_deleted: bool) {
        assert!(!self.subtrees.contains_key(&nk_offset), "KeyNode at offset 0x{:08x} is already in the set of subtrees", nk_offset.0);
        assert!(!self.entries.contains_key(&nk_offset));

        let parent_offset = nk.parent;
        let entry = Rc::new(RefCell::new(RegTreeEntry::new(nk_offset, nk, is_deleted)));

        // check if the parent of the current node has already been added.
        // If yes, than put the current node below of it. If not, add the
        // current node at the root level (which can contain more than one nodes)
        match self.entries.get(&parent_offset) {
            Some(parent_entry) => {
                assert!(!self.missing_parents.contains_key(&parent_offset));
                parent_entry.borrow_mut().add_child(Rc::clone(&entry))
            }
            None => {
                self.subtrees.insert(nk_offset, Rc::clone(&entry));
                self.add_child_that_misses_parent(Rc::clone(&entry), parent_offset);
            }
        }

        // check if the current node has children which have already been
        // added. If yes, those children should've been at the root level
        // until now and must be reordered
        if let Some(children) = self.missing_parents.remove(&nk_offset) {
            for child in children.into_iter() {
                entry.borrow_mut().add_child(child);
            }
        }

        self.entries.insert(nk_offset, entry);
    }

    fn add_child_that_misses_parent(
        &mut self,
        child: Rc<RefCell<RegTreeEntry>>,
        parent_offset: Offset,
    ) {
        match self.missing_parents.get_mut(&parent_offset) {
            Some(parent) => {
                parent.push(child);
            }
            None => {
                self.missing_parents.insert(parent_offset, vec![child]);
            }
        }
    }
}
