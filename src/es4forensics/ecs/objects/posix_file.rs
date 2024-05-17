use std::collections::HashMap;
use anyhow::Result;

use crate::common::bodyfile::{Bodyfile3Line, BehavesLikeI64};
use chrono_tz::Tz;
use serde::Serialize;

use crate::es4forensics::{timestamp::Timestamp, ecs::{timeline_object::TimelineObject, ecs_builder::EcsBuilder}};
use crate::es4forensics::ecs::File;

#[derive(Serialize)]
pub struct PosixFile {
    name: String,
    inode: String,
    uid: u64,
    gid: u64,
    size: u64,
    atime: Option<Timestamp>,
    mtime: Option<Timestamp>,
    ctime: Option<Timestamp>,
    crtime: Option<Timestamp>,
}

impl PosixFile {
    fn load_timestamp<TS>(ts: &TS, tz: &Tz) -> Result<Option<Timestamp>> where TS: BehavesLikeI64 {
        match ts.as_ref() {
            None => Ok(None),
            Some(ts) => {
                Ok(Some((*ts, tz).try_into()?))
            }
        }
    }

    // fn generate_macb(&self, reference_ts: &Timestamp) -> Macb {
    //     let mut macb = Macb::default();
    //     if let Some(t) = self.mtime.as_ref() {
    //         macb.modified = t == reference_ts;
    //     }
    //     if let Some(t) = self.atime.as_ref() {
    //         macb.accessed = t == reference_ts;
    //     }
    //     if let Some(t) = self.ctime.as_ref() {
    //         macb.changed = t == reference_ts;
    //     }
    //     if let Some(t) = self.crtime.as_ref() {
    //         macb.created = t == reference_ts;
    //     }
    //     macb
    // }

    fn add_builder_to(&self, docs: &mut HashMap<Timestamp, anyhow::Result<EcsBuilder>>, ts: &Option<Timestamp>) {
        if let Some(t) = ts.as_ref() {
            //let macb = self.generate_macb(t);
            if ! docs.contains_key(t) {
                let file = File::from(self.name.clone())
                    .with_inode(self.inode.clone())
                    .with_uid(self.uid)
                    .with_gid(self.gid)
                    .with_size(self.size)
                    .with_mtime(self.mtime.clone())
                    .with_accessed(self.atime.clone())
                    .with_ctime(self.ctime.clone())
                    .with_created(self.crtime.clone());
                let builder = EcsBuilder::new(self.name.clone(), t.clone())
                    .with_additional_tag("bodyfile")
                    .with_file(file);
                docs.insert(t.clone(), builder);
            }
        }
    }
}

impl TimelineObject for PosixFile {}

impl IntoIterator for PosixFile {
    type Item = anyhow::Result<EcsBuilder>;
    type IntoIter = std::collections::hash_map::IntoValues<Timestamp, Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut docs = HashMap::new();
        self.add_builder_to(&mut docs, &self.mtime);
        self.add_builder_to(&mut docs, &self.atime);
        self.add_builder_to(&mut docs, &self.ctime);
        self.add_builder_to(&mut docs, &self.crtime);
        
        docs.into_values()
    }
}

impl TryFrom<Bodyfile3Line> for PosixFile {
    type Error = anyhow::Error;
    fn try_from(bfline: Bodyfile3Line) -> Result<Self> {
        let src_tz = Tz::UTC;
        Self::try_from((&bfline, &src_tz))
    }
}

impl TryFrom<&Bodyfile3Line> for PosixFile {
    type Error = anyhow::Error;
    fn try_from(bfline: &Bodyfile3Line) -> Result<Self> {
        let src_tz = Tz::UTC;
        Self::try_from((bfline, &src_tz))
    }
}

impl TryFrom<(&Bodyfile3Line, &Tz)> for PosixFile {
    type Error = anyhow::Error;
    fn try_from((bfline, src_tz): (&Bodyfile3Line, &Tz)) -> Result<Self> {
        Ok(Self {
            name: bfline.get_name().to_string(),
            inode: bfline.get_inode().to_string(),
            uid: *bfline.get_uid(),
            gid: *bfline.get_gid(),
            size: *bfline.get_size(),
            atime: Self::load_timestamp(bfline.get_atime(), src_tz)?,
            mtime: Self::load_timestamp(bfline.get_mtime(), src_tz)?,
            ctime: Self::load_timestamp(bfline.get_ctime(), src_tz)?,
            crtime: Self::load_timestamp(bfline.get_crtime(), src_tz)?,
        })
    }
}

impl TryFrom<&str> for PosixFile {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bfline: Bodyfile3Line = value.try_into()?;
        bfline.try_into()
    }
}