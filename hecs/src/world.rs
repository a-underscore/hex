use crate::{Component, Entity, Id, System};
use glium::glutin::event::Event;
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

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

    pub fn add(&mut self, id: &Id, entity: &Rc<RefCell<Entity>>) {
        self.entities.insert(id.clone(), entity.clone());
    }

    pub fn get(&self, id: &Id) -> Option<Rc<RefCell<Entity>>> {
        self.entities.get(id.as_ref()).and_then(|e| Some(e.clone()))
    }

    pub fn get_all_with(
        &self,
        ids: &[&Id],
    ) -> Vec<(Rc<RefCell<Entity>>, Vec<Rc<RefCell<dyn Component>>>)> {
        self.entities
            .values()
            .filter_map(|e| Some((e.clone(), e.borrow().get_all(ids)?)))
            .collect()
    }

    pub fn remove(&mut self, id: &Id) {
        self.entities.remove(id.as_ref());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System,
    {
        self.systems.insert(system.borrow().id(), system.clone());
    }

    pub fn remove_system(&mut self, id: &Id) {
        self.systems.remove(id.as_ref());
    }

    pub fn init_systems(&mut self) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_init(self);
        }
    }

    pub fn update_systems(&mut self, event: &Event<()>, delta: Duration) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_update(self, event, delta);
        }
    }

    pub fn get_entities<'a>(&'a self) -> &'a HashMap<Id, Rc<RefCell<Entity>>> {
        &self.entities
    }
}
