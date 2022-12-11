use super::{cast, new, AsAny, Component, Id, Type};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Entity {
    pub components: BTreeMap<Id, (Id, Type<dyn AsAny>)>,
}

impl Entity {
    pub fn new() -> Type<Self> {
        new(Self {
            components: BTreeMap::new(),
        })
    }

    pub fn components(&self) -> Vec<(Id, Type<dyn AsAny>)> {
        self.components.values().cloned().collect()
    }

    pub fn add_generic(&mut self, c @ (id, _): &(Id, Type<dyn AsAny>)) {
        self.components.insert(id.clone(), c.clone());
    }

    pub fn add<C>(&mut self, c: &Type<C>)
    where
        C: Component + 'static,
    {
        self.add_generic(&(C::id(), c.clone()));
    }

    pub fn generic(&self, id: &Id) -> Option<&(Id, Type<dyn AsAny>)> {
        self.components.get(id)
    }

    pub fn get<C>(&self) -> Option<Type<C>>
    where
        C: Component,
    {
        self.generic(&C::id()).map(|(_, c)| cast(c))
    }

    pub fn all(&self, ids: &[&Id]) -> Vec<(Id, Type<dyn AsAny>)> {
        ids.iter()
            .filter_map(|id| self.generic(id).cloned())
            .collect()
    }

    pub fn remove_generic(&mut self, id: &Id) -> Option<(Id, Type<dyn AsAny>)> {
        self.components.remove(id)
    }

    pub fn remove<C>(&mut self) -> Option<(Id, Type<dyn AsAny>)>
    where
        C: Component,
    {
        self.remove_generic(&C::id())
    }
}
