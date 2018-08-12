///! ECS root module
///! =======
///! This module contains the word structure, the rest of the ecs is in the submodules
///! -----------

mod component;
mod entity;
mod views;

use std::collections::HashSet;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

pub struct World
{
    indexer : entity::EntityIndexer,
    component_factory: component::ComponentFactory,
    hierarchy: entity::AdjHashMap<entity::Entity>,
}

impl World
{
    //moves all the children from one parent to a new one
    fn move_children(&mut self, old_parent: &u64, new_parent: &u64) -> bool
    {
        let mut new_parent_children = self.hierarchy.remove(&new_parent)/*.unwrap().1*/;
        if(new_parent_children.is_none())
        {
            return false;
        }
        let mut new_parent_children = new_parent_children.unwrap().1;
        for child in self.hierarchy.get(&old_parent).unwrap().1.iter()
        {
            new_parent_children.insert(child.clone());
        }
        true
    }

    //deletes entities recursively
    fn recursive_delete(&mut self, parent: &u64) -> bool
    {
        let children = self.hierarchy.remove(parent);
        if(children.is_none())
        {
            return false;
        }

        let children = children.unwrap();
        for child in children.1.iter()
        {
           self.recursive_delete(child);
        }
        true
    }

    /// this function will create and add ann entity to the world as a chilf of the specified parent
    /// # Errors
    /// if the specified parent does not exist it will return an Error, otherwise an Ok containing the Entity index
    pub fn add_entity(& mut self, parent: u64) -> Result<u64, &str>
    {
        let new_index: u64 = self.indexer.get_index();
        match self.hierarchy.get_mut(&parent)
        {
            Some(parent) => {
                parent.1.insert(new_index);
            }
            None => return Err("unable to insert the entity in the hierarchy, parent not found")
        }
        self.hierarchy.insert(new_index, (entity::Entity::new(), HashSet::new()));
        Ok(new_index)
    }

    /// this function will destroy the specified entity and assign it's children to the spcified new parent.
    ///
    /// If you instead want to destroy all of the children use rem_entity_recursive
    /// # Errors
    /// if the spcified entity does not exist it will return an Error, otherwise an empty Ok
    pub fn rem_entity(& mut self, entity:u64, new_parent: u64) -> Result<(), &str>
    {
        //moves all children
        if self.move_children(&entity, &new_parent)
        {
            Ok(())
        }
        else
        {
            Err("unable to find the entity to remove")
        }
    }

    /// this function will destroy the specified entity and it's children.
    ///
    /// If you instead want to assign all of the children to a specified entity use rem_entity
    /// # Errors
    /// if the spcified entity does not exist it will return an Error, otherwise an empty Ok
    pub fn rem_entity_recursive(& mut self, entity:u64) -> Result<(), &str>
    {
        if self.recursive_delete(&entity)
        {
            return Ok(())
        }
        else
        {
            return Err("unable to find the entity to remove")
        }
    }
}