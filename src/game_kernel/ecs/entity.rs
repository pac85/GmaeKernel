use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Deref;
use parking_lot::Mutex;
use evmap;
use evmap::ShallowCopy;

use super::component;

pub type keytype = u64;

#[derive(Hash, Eq, PartialEq)]
pub struct EntityIndexer
{
    last_index :u64,
}

impl EntityIndexer
{
    pub fn new() -> Self
    {
        Self {last_index: 1}    //we start from 1 because 0 is the scene root
    }
    pub fn get_index(&mut self) -> u64
    {
        self.last_index += 1;
        self.last_index
    }
}

/*#[derive(PartialEq, Eq, Clone)]
pub struct Entity
{
    components: Vec<Box<component::Component>>,
}

impl Entity
{
    pub fn new() -> Self
    {
        Self{components: Vec::new()}
    }
}

impl ShallowCopy for Entity
{
    unsafe fn shallow_copy(&mut self) -> Self {
        &*self
    }
}*/

pub struct Entities{
    r: evmap::ReadHandle<keytype, component::ComponentBox>,
    w: Mutex<evmap::WriteHandle<keytype, component::ComponentBox>>,
}

pub struct AdjHashMap
{
    pub r: evmap::ReadHandle<keytype, keytype>,
    pub w: Mutex<evmap::WriteHandle<keytype, keytype>>,
    indexer: EntityIndexer,
}

impl AdjHashMap
{
    pub fn new() -> Self
    {
        let (r, mut w) = evmap::new();
        Self{r, w: Mutex::new(w), indexer: EntityIndexer::new()}
    }
    pub fn exists(&self, index: &u64) -> bool
    {
        if *index == 0u64{
            return true;
        }
        self.r.get_and(index, |entity| ()).is_some()
    }

    pub fn insert_entity(&mut self, parent: &u64) -> Option<u64>
    {
        if !self.exists(parent){
            return None;
        }
        let mut hierarchy_w = self.w.lock();
        let hierarchy_r = &self.r;

        let new_index: u64 = self.indexer.get_index();

        hierarchy_w.insert(parent.clone(), new_index);
        Some(new_index)
    }
}