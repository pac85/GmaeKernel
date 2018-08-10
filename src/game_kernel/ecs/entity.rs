use std::collections::HashSet;
use std::collections::HashMap;

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

pub type AdjHashMap<T> = HashMap<u64, (T, HashSet<u64>)>;
