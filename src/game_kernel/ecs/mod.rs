//! ECS root module
//! =======
//! This module contains the word structure, the rest of the ecs is in the submodules
//! -----------

pub mod entity;
pub mod component;
pub mod system;

mod views;

use std::collections::HashSet;
use std::collections::HashMap;
use evmap::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct World
{
    component_factory: component::ComponentFactory,
    hierarchy: entity::AdjHashMap,
    views: Vec <views::ViewRef>,
}

impl World
{
    //moves all the children from one parent to a new one, return false if it can't find
    //the new parent, true otherwise
    fn move_children(&mut self, old_parent: &u64, new_parent: &u64) -> bool
    {
        let mut hierarchy_w = self.hierarchy.w.lock();
        let mut hierarchy_r = self.hierarchy.r;
        //checks if the new parent exists
        if !self.hierarchy.exists(new_parent)
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
    fn recursive_delete<F>(&mut self, parent: &u64, on_delete: F) -> bool
        where F: FnOnce(u64)
    {
        /*
        let children = self.hierarchy.remove(parent);
        if children.is_none()
        {
            return false;
        }

        let children = children.unwrap();
        for child in children.1.iter()
        {
           self.recursive_delete(child);
        }
        */
        let mut hierarchy_w = self.hierarchy.w.lock();
        //if the parent hasn't been found we return false
        if !self.hierarchy.exists(parent) {
            return false;
        }
        //if the current parent is a lief the function returns
        if hierarchy_w.get_and(parent, |children| children.is_empty()).unwrap(){
            return true;
        }
        hierarchy_w.get_and(parent, |children| {
            for child in children{
                on_delete(child.clone());
                self.recursive_delete(child, on_delete);
            }
        });
        //at  this point we lock the mutex, if that happened before the recursive call we would
        // get a dead lock
        let mut hierarchy_r = self.hierarchy.r;
        hierarchy_w.clear(parent.clone());
        true
    }

    //this funvtion updates the views when an entity is added
    fn update_views_on_added(&self, entity_index: u64)
    {
        for view in self.views.iter()
        {
            (view.on_enity_added)(entity_index);
        }
    }

    //this funvtion updates the views when an entity is added
    fn update_views_on_removed(&self, entity_index: u64)
    {
        for view in self.views.iter()
            {
                (view.on_enity_removed)(entity_index);
            }
    }

    /// This function will register a view.
    /// # Examples
    /// '''
    /// //TestView implements View
    /// register_view::<TestView>()
    /// '''
    pub fn register_view<T>(&mut self)
        where T: views::View<Item=entity::keytype> + 'static
    {
        self.views.push(views::ViewRef::new::<T>());
        T::on_register(&self);
    }

    /// this function will create and add ann entity to the world as a child of the specified parent
    /// # Errors
    /// if the specified parent does not exist it will return an Error, otherwise an Ok containing the Entity index
    pub fn add_entity(& mut self, parent: u64) -> Result<u64, &str>
    {
        match self.hierarchy.insert_entity(&parent)
        {
            Some(index) => Ok(index),
            None        => Err("unable to insert the entity in the hierarchy, parent not found"),
        }
    }

    /// this function will destroy the specified entity and assign it's children to the specified new parent.
    ///
    /// If you instead want to destroy all of the children use rem_entity_recursive
    /// # Errors
    /// if the specified entity does not exist it will return an Error, otherwise an empty Ok
    pub fn rem_entity(& mut self, entity:u64, new_parent: u64) -> Result<(), &str>
    {
        //moves all children
        if self.move_children(&entity, &new_parent)
        {
            self.update_views_on_removed(entity);
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
        if self.recursive_delete(&entity, |entity_index|self.update_views_on_removed(entity))
        {
            return Ok(())
        }
        else
        {
            return Err("unable to find the entity to remove")
        }
    }
}