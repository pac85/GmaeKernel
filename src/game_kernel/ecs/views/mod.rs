use super::entity;

pub trait View: Iterator<Item=entity::Entity>
{
    fn on_enity_added(&mut self, entity: &entity::Entity);
    fn on_enity_removed(&mut self, entity: &entity::Entity);
}

pub trait ImmediateView: Iterator<Item=entity::Entity>
{
    fn new(enity_map: &entity::AdjHashMap<entity::Entity>) -> Box< ImmediateView<Item=entity::Entity> >
        where Self: Sized;
}