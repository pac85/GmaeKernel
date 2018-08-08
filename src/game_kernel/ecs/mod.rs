use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct EnitityIndexer
{
    index :u64,
}

impl EnitityIndexer
{
    pub fn get_index(&mut self) -> u64
    {
        self.index += 1;
        self.index
    }
}

type AdjHashMap<T> = HashMap<u64, (T, HashSet<u64>)>;
//tree structure of entities
struct EntityHierarchy
{
    adj_hash_map: AdjHashMap<Entity>,
}

pub struct World
{
    indexer :EnitityIndexer,
    free_entities: Vec<Entity>,         //entities which aren't either parent or childrens
    hierarchies: Vec<EntityHierarchy>   //basically a forest structure
}

pub struct Entity
{
    coponents: Vec<Box<Component>>,
}

pub trait Component
{

}