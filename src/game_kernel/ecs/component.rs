use std::collections::HashMap;
use evmap::ShallowCopy;

pub trait Component
{
    fn get_name() -> String where Self: Sized;
    fn get_builder() -> fn()->Box<Component> where Self: Sized;
}

pub struct ComponentFactory
{
    components_map: HashMap<String, fn() ->Box<Component>>,
}

impl ComponentFactory
{
    pub fn register<T>(&mut self)
        where T: Component,
    {
        self.components_map.insert(T::get_name(), T::get_builder());
    }

    pub fn instantiate(&mut self, name: &str) -> Option<Box<Component>>
    {
        match self.components_map.get(name)
            {
                Some(builder_fn) => Some(builder_fn()),
                None => None,
            }
    }
}