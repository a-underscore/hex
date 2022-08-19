use crate::{Component, Entity, Id, System};
use glium::glutin::event::Event;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
    time::Duration,
};

pub struct World {
    entities: HashMap<Id, Rc<RefCell<Entity>>>,
    systems: HashMap<Id, Rc<RefCell<dyn System>>>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: HashMap::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn change_id(&mut self, old: &Id, new: &Id) {
        self.remove(old).and_then(|(_, e)| {
            self.add(new, &e);

            Some(())
        });
    }

    pub fn add(&mut self, id: &Id, entity: &Rc<RefCell<Entity>>) {
        self.entities.insert(id.clone(), entity.clone());
    }

    pub fn get_entities<'a>(&'a self) -> &'a HashMap<Id, Rc<RefCell<Entity>>> {
        &self.entities
    }

    pub fn get(&self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        self.entities
            .get(id.as_ref())
            .and_then(|e| Some((id.clone(), e.clone())))
    }

    pub fn get_all(&self, id: &Id) -> Vec<((Id, Rc<RefCell<Entity>>), Rc<RefCell<dyn Component>>)> {
        self.entities
            .iter()
            .filter_map(|(i, e)| {
                Some((
                    (i.clone(), e.clone()),
                    e.borrow().get(id).and_then(|c| Some(c.clone()))?,
                ))
            })
            .collect()
    }

    pub fn get_all_with(
        &self,
        ids: &[&Id],
    ) -> Vec<((Id, Rc<RefCell<Entity>>), Vec<Rc<RefCell<dyn Component>>>)> {
        self.entities
            .iter()
            .filter_map(|(id, e)| Some(((id.clone(), e.clone()), e.borrow().get_all(ids)?)))
            .collect()
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<Entity>>)> {
        self.entities
            .remove(id.as_ref())
            .and_then(|e| Some((id.clone(), e)))
    }

    pub fn add_generic_system(&mut self, system: Rc<RefCell<dyn System>>) {
        self.systems
            .insert(system.borrow().get_id(), system.clone());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System,
    {
        self.add_generic_system(system.clone())
    }

    pub fn get_systems<'a>(&'a self) -> &'a HashMap<Id, Rc<RefCell<dyn System>>> {
        &self.systems
    }

    pub fn get_system<'a>(&'a self, id: &Id) -> Option<&'a Rc<RefCell<dyn System>>> {
        self.systems.get(id)
    }

    pub fn get_system_ref<S>(&self, id: &Id) -> Option<Ref<S>>
    where
        S: System,
    {
        self.get_system(id)
            .and_then(|s| Ref::filter_map(s.borrow(), |s| s.as_any_ref().downcast_ref::<S>()).ok())
    }

    pub fn get_system_mut<S>(&self, id: &Id) -> Option<RefMut<S>>
    where
        S: System,
    {
        self.get_system(id).and_then(|s| {
            RefMut::filter_map(s.borrow_mut(), |s| s.as_any_mut().downcast_mut::<S>()).ok()
        })
    }

    pub fn remove_system(&mut self, id: &Id) -> Option<Rc<RefCell<dyn System>>> {
        self.systems.remove(id.as_ref())
    }

    pub fn init_systems(&mut self) {
        for s in self.systems.clone().values() {
            s.borrow_mut().init(self);
        }
    }

    pub fn update_systems(&mut self, event: &Event<()>, delta: Duration) {
        for s in self.systems.clone().values() {
            s.borrow_mut().update(self, event, delta);
        }
    }
}
