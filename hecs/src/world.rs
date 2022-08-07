use crate::{Entity, Id, System};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    pub entities: Vec<Rc<RefCell<Entity>>>,
    pub systems: HashMap<Id, Rc<dyn System>>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: Vec::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn add(&mut self, entity: &Rc<RefCell<Entity>>) {
        self.entities.push(entity.clone());
    }

    pub fn get(&self, id: &Id) -> Option<Rc<RefCell<Entity>>> {
        self.entities
            .iter()
            .cloned()
            .find(|e| *e.borrow().id() == **id)
    }

    pub fn remove(&mut self, id: Id) {
        self.entities = self
            .entities
            .iter()
            .cloned()
            .filter(|e| *e.borrow().id() != **id)
            .collect();
    }

    pub fn add_system<S>(&mut self, system: &Rc<S>)
    where
        S: System,
    {
        self.systems
            .insert(system.id(), system.clone() as Rc<dyn System>);
    }

    pub fn get_system<S>(&self, id: &Id) -> Option<Rc<S>>
    where
        S: System,
    {
        self.systems
            .get(id)
            .and_then(|s| s.clone().as_any().downcast::<S>().ok())
    }

    pub fn remove_system<S>(&mut self, id: Id) {
        self.systems.remove(id.as_ref());
    }

    pub fn init(&mut self) {
        for s in self.systems.clone().values() {
            s.on_init(self);
        }
    }

    pub fn update(&mut self) {
        for s in self.systems.clone().values() {
            s.on_update(self);
        }
    }
}
