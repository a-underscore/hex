use crate::{Entity, Id, System};
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

    pub fn remove(&mut self, id: &Id) {
        self.entities.remove(id.as_ref());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System,
    {
        self.systems
            .insert(system.borrow().id(), system.clone() as Rc<RefCell<S>>);
    }

    pub fn remove_system(&mut self, id: &Id) {
        self.systems.remove(id.as_ref());
    }

    pub fn init(&mut self) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_init(self);
        }
    }

    pub fn update(&mut self, event: &Event<()>, delta: Duration) {
        for s in self.systems.clone().values() {
            s.borrow_mut().on_update(self, event, delta);
        }
    }

    pub fn entities<'a>(&'a self) -> &'a HashMap<Id, Rc<RefCell<Entity>>> {
        &self.entities
    }
}
