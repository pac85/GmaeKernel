pub type Keytype = u64;

extern crate parking_lot;
extern crate evmap;

use parking_lot::Mutex;
use evmap::ShallowCopy;

#[derive(Hash, Eq, PartialEq)]
pub struct HierarchyIndexer
{
    last_index :u64,
}

impl HierarchyIndexer
{
    pub fn new() -> Self
    {
        Self {last_index: 1}    //we start from 1 because 0 is the root
    }
    pub fn get_index(&mut self) -> u64
    {
        self.last_index += 1;
        self.last_index
    }
}

pub struct AdjHashMap
{
    pub r: evmap::ReadHandle<Keytype, Keytype>,
    pub w: Mutex<evmap::WriteHandle<Keytype, Keytype>>,
    indexer: HierarchyIndexer,
}

impl AdjHashMap
{
    pub fn new() -> Self
    {
        let (r, mut w) = evmap::new();
        Self{r, w: Mutex::new(w), indexer: HierarchyIndexer::new()}
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

pub mod utils
{
    use crate::hierarchy::*;
    //moves all the children from one parent to a new one, return false if it can't find
    //the new parent, true otherwise
    pub fn move_children(hierarchy: &mut AdjHashMap, old_parent: &u64, new_parent: &u64) -> bool
    {
        let mut hierarchy_w = hierarchy.w.lock();
        let hierarchy_r = &hierarchy.r;
        //checks if the new parent exists
        if !hierarchy.exists(new_parent)
            {
                return false;
            }
        //copies all the children of old_parent to the new parent
        hierarchy_r.get_and(&old_parent, |children|{
            for child in children {
                hierarchy_w.insert(new_parent.clone(), child.clone());
            }
        });
        //removes the children from the old parent
        hierarchy_w.clear(old_parent.clone());
        hierarchy_w.flush();

        true
    }

    //deletes entities recursively
    pub fn recursive_delete(hierarchy: &mut AdjHashMap, parent: &u64, on_removed: &Fn(&Keytype)) -> bool
    {
        let hierarchy_r = &hierarchy.r.clone();
        //if the parent hasn't been found we return false
        if !hierarchy.exists(parent) {
            return false;
        }
        //if the current parent is a lief the function returns
        if hierarchy_r.get_and(parent, |children| children.is_empty()).unwrap(){
            return true;
        }
        hierarchy_r.get_and(parent, |children| {
            for child in children{
                on_removed(child);
                recursive_delete(hierarchy, child, on_removed);
            }
        });
        //at  this point we lock the mutex, if that happened before the recursive call we would
        // get a dead lock
        let mut hierarchy_w = hierarchy.w.lock();
        hierarchy_w.clear(parent.clone());
        true
    }
}
