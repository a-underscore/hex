use super::{AsAny, Component, Entity, Id, System};
use glium::glutin::event::Event;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

#[derive(Clone)]
pub struct World {
    entities: HashMap<Id, (Id, Rc<RefCell<Entity>>)>,
    systems: HashMap<Id, (Id, Rc<RefCell<dyn System>>)>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: HashMap::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn change_id(&mut self, old: &Id, new: &Id) {
        if let Some((_, e)) = self.remove(old) {
            self.add(&(new.clone(), e));
        };
    }

    pub fn add(&mut self, e @ (id, _): &(Id, Rc<RefCell<Entity>>)) {
        self.entities.insert(id.clone(), e.clone());
    }

    pub fn get_entities(&self) -> &HashMap<Id, (Id, Rc<RefCell<Entity>>)> {
        &self.entities
    }

    pub fn get(&self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        Some(self.entities.get(id.as_ref())?.clone())
    }

    pub fn get_all(
        &self,
        ids: &[&Id],
    ) -> Vec<((Id, Rc<RefCell<Entity>>), Vec<(Id, Rc<RefCell<dyn AsAny>>)>)> {
        self.entities
            .values()
            .filter_map(|e @ (_, en)| Some((e.clone(), en.try_borrow().ok()?.get_all(ids)?)))
            .collect()
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        self.entities.remove(id.as_ref())
    }

    pub fn add_generic_system(&mut self, s @ (id, _): &(Id, Rc<RefCell<dyn System>>)) {
        self.systems.insert(id.clone(), s.clone());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System + Component + 'static,
    {
        self.add_generic_system(&(S::get_id(), system.clone()))
    }

    pub fn get_systems(&self) -> &HashMap<Id, (Id, Rc<RefCell<dyn System>>)> {
        &self.systems
    }

    pub fn get_system(&self, id: &Id) -> Option<&(Id, Rc<RefCell<dyn System>>)> {
        self.systems.get(id)
    }

    pub fn get_system_mut(&mut self, id: &Id) -> Option<&mut (Id, Rc<RefCell<dyn System>>)> {
        self.systems.get_mut(id)
    }

    pub fn get_system_ref<S>(&self, id: &Id) -> Option<(Id, Ref<S>)>
    where
        S: System + 'static,
    {
        self.get_system(id).and_then(|(id, s)| {
            Some((
                id.clone(),
                Ref::filter_map(s.try_borrow().ok()?, |s| s.as_any_ref().downcast_ref::<S>())
                    .ok()?,
            ))
        })
    }

    pub fn get_system_ref_mut<S>(&self, id: &Id) -> Option<(Id, RefMut<S>)>
    where
        S: System + 'static,
    {
        self.get_system(id).and_then(|(id, s)| {
            Some((
                id.clone(),
                RefMut::filter_map(s.try_borrow_mut().ok()?, |s| {
                    s.as_any_mut().downcast_mut::<S>()
                })
                .ok()?,
            ))
        })
    }

    pub fn remove_system(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<dyn System>>)> {
        self.systems.remove(id.as_ref())
    }

    pub fn update_systems(&mut self, event: &Event<()>, delta: Duration) -> anyhow::Result<()> {
        for (_, s) in self.systems.clone().values() {
            s.try_borrow_mut()?.update(self, event, delta)?;
        }

        Ok(())
    }
}
