use std::collections::HashMap;
use evmap::ShallowCopy;
use std::any::Any;
use std::ops::Deref;

pub trait Component
{
    fn get_name() -> String where Self: Sized;
    fn get_builder() -> fn()->Box<Component> where Self: Sized;
}

pub trait ComponentEq
{
    fn equals_a(&self, other: &'static Component) -> bool;
}

impl<S: 'static + Component + PartialEq + Eq> ComponentEq for S
{
    fn equals_a(&self, other: &'static Component) -> bool {
        // Do a type-safe casting. If the types are different,
        // return false, otherwise test the values for equality.
        (&other as &Any)
            .downcast_ref::<S>()
            .map_or(false, |a| self == a)
    }
}

pub struct ComponentBox<T = Component + PartialEq + Eq>
{
    boxed_component: Box<T>,
}

impl<S: 'static + Component + PartialEq + Eq> PartialEq for ComponentBox<S>
{
    fn eq(&self, other: &ComponentBox) -> bool {
        (**self).equals_a(other)
    }
}

impl<S: 'static + Component + PartialEq + Eq> Eq for ComponentBox<S>{}

impl From<Box<Component>> for ComponentBox<Component>
{
    fn from(boxed_component: Box<Component>) -> Self{
        return Self{boxed_component}
    }
}

impl ShallowCopy for ComponentBox<Component>
{
    unsafe fn shallow_copy(&mut self) -> Self {
        self.boxed_component.shallow_copy()
    }
}

impl Deref for ComponentBox<Component> {
    type Target = Component;

    fn deref(&self) -> &'static Component {
        self.boxed_component
    }
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