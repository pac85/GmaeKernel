use super::entity;

pub trait View: Iterator<Item=entity::Entity>
{
    fn on_register(enity_map: &entity::AdjHashMap<entity::Entity>);
    fn on_enity_added(u64);
    fn on_enity_removed(u64);
}

//used by world to store the views that it registers
pub struct ViewRef
{
    pub on_register:        fn(enity_map: &entity::AdjHashMap<entity::Entity>),
    pub on_enity_added:     fn(u64),
    pub on_enity_removed:   fn(u64),
}

impl ViewRef
{
    pub fn new<T>() -> Self
        where T: View
    {
        Self{on_register: T::on_register, on_enity_added: T::on_enity_added, on_enity_removed: T::on_enity_removed}
    }
}

pub trait ImmediateView: Iterator<Item=entity::Entity>
{
    fn new(enity_map: &entity::AdjHashMap<entity::Entity>) -> Box< ImmediateView<Item=entity::Entity> >
        where Self: Sized;
}