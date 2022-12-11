use super::{cast, new, Component, Entity, Id, System, Type};
use glium::glutin::event::Event;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct World {
    pub entities: BTreeMap<Id, (Id, Type<Entity>)>,
    pub systems: BTreeMap<Id, (Id, Type<dyn System>)>,
}

impl World {
    pub fn new() -> Type<Self> {
        new(Self {
            entities: BTreeMap::new(),
            systems: BTreeMap::new(),
        })
    }

    pub fn entities(&self) -> Vec<(Id, Type<Entity>)> {
        self.entities.values().cloned().collect()
    }

    pub fn add(&mut self, e @ (id, _): &(Id, Type<Entity>)) {
        self.entities.insert(id.clone(), e.clone());
    }

    pub fn get(&self, id: &Id) -> Option<(Id, Type<Entity>)> {
        Some(self.entities.get(id)?.clone())
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Type<Entity>)> {
        self.entities.remove(id)
    }

    pub fn systems(&self) -> Vec<(Id, Type<dyn System>)> {
        self.systems.values().cloned().collect()
    }

    pub fn add_generic_system(&mut self, s @ (id, _): &(Id, Type<dyn System>)) {
        self.systems.insert(id.clone(), s.clone());
    }

    pub fn add_system<S>(&mut self, s: &Type<S>)
    where
        S: System + Component + 'static,
    {
        self.add_generic_system(&(S::id(), s.clone()))
    }

    pub fn system_generic(&self, id: &Id) -> Option<&(Id, Type<dyn System>)> {
        self.systems.get(id)
    }

    pub fn system<S>(&self) -> Option<Type<S>>
    where
        S: System + Component,
    {
        self.system_generic(&S::id()).map(|(_, s)| cast(s))
    }

    pub fn remove_generic_system(&mut self, id: &Id) -> Option<(Id, Type<dyn System>)> {
        self.systems.remove(id)
    }

    pub fn remove_system<S>(&mut self) -> Option<(Id, Type<dyn System>)>
    where
        S: Component,
    {
        self.remove_generic_system(&S::id())
    }

    pub fn update(&mut self, event: &Event<()>) -> anyhow::Result<()> {
        for (_, s) in self.systems.clone().values() {
            s.try_borrow_mut()?.update(self, event)?;
        }

        Ok(())
    }
}
