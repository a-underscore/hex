use crate::{Entity, Id, System};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    entities: Vec<Rc<RefCell<Entity>>>,
    systems: HashMap<Id, Rc<RefCell<dyn System>>>,
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

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System,
    {
        self.systems
            .insert(system.borrow().id(), system.clone() as Rc<RefCell<S>>);
    }

    pub fn remove_system(&mut self, id: Id) {
        self.systems.remove(id.as_ref());
    }

    pub fn init(&mut self) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_init(self);
        }
    }

    pub fn update(&mut self) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_update(self);
        }
    }

    pub fn entities<'a>(&'a self) -> &'a Vec<Rc<RefCell<Entity>>> {
        &self.entities
    }
}
