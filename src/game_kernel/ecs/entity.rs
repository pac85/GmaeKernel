use std::collections::HashSet;
use std::collections::HashMap;
use parking_lot::Mutex;
use evmap;
use evmap::ShallowCopy;

impl<T> ShallowCopy for HashSet<T>
{
    unsafe fn shallow_copy(&mut self) -> Self {
        &*self
    }
}

use super::component;

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

pub struct AdjHashMap<T>
    where T: Eq + evmap::ShallowCopy,
{
    pub r: evmap::ReadHandle<u64, (T, HashSet<u64>)>,
    pub w: Mutex<evmap::WriteHandle<u64, (T, HashSet<u64>)>>,
}

impl<T> AdjHashMap<T>
{
    pub fn new() -> Self
    {
        let (r, mut w) = evmap::new();
        Self{r,w,}
    }
}