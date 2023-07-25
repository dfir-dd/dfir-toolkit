use std::{rc::Rc, cell::{RefCell, Ref}, slice::Iter};
use nt_hive2::{Offset, KeyNode};

pub (crate) struct RegTreeEntry {
    offset: Offset,
    nk: KeyNode,
    is_deleted: bool,
    children: Vec<Rc<RefCell<Self>>>,
}

impl RegTreeEntry {
    pub fn new(offset: Offset, nk: KeyNode, is_deleted: bool) -> Self {
        Self { offset, nk, is_deleted, children: Vec::new() }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        self.children.push(child);
    }

    pub fn offset(&self) -> &Offset {
        &self.offset
    }

    pub fn nk(&self) -> &KeyNode {
        &self.nk
    }

    pub fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    pub fn children(&self) -> Children {
        Children{
            children: self.children.iter()
        }
    }
}

pub (crate) struct Children<'a> {
    children: Iter<'a, Rc<RefCell<RegTreeEntry>>>
}

impl<'a> Iterator for Children<'a> {
    type Item = Ref<'a, RegTreeEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.children.next().map(|r| r.borrow())
    }
}