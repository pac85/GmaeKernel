//! ECS root module
//! =======
//! This module contains the word structure, the rest of the ecs is in the submodules
//! -----------

pub mod entity;
pub mod component;
pub mod system;

mod views;

use game_kernel_utils::hierarchy;

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
    //this function updates the views when an entity is added
    fn update_views_on_added(&self, entity_index: &u64)
    {
        for view in self.views.iter()
        {
            (view.on_enity_added)(entity_index);
        }
    }

    //this function updates the views when an entity is added
    fn update_views_on_removed(&self, entity_index: &u64)
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
        where T: views::View<Item=entity::Keytype> + 'static
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
    pub fn rem_entity(& mut self, entity: &u64, new_parent: u64) -> Result<(), &str>
    {
        //moves all children
        if hierarchy::utils::move_children(&mut self.hierarchy,&entity, &new_parent)
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
    /// if the specified entity does not exist it will return an Error, otherwise an empty Ok
    pub fn rem_entity_recursive(& mut self, entity:u64) -> Result<(), &str>
    {
        let raw_self = self as *const Self;
        let on_removed = |entity_index: &hierarchy::Keytype| {unsafe {(*raw_self).update_views_on_removed(&entity_index);}};
        if hierarchy::utils::recursive_delete(&mut self.hierarchy,&entity, &on_removed)
        {
            return Ok(())
        }
        else
        {
            return Err("unable to find the entity to remove")
        }
    }
}