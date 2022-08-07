use crate::{AsAny, Entity, Id, System};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    pub entities: HashMap<Id, Rc<RefCell<Entity>>>,
    pub systems: HashMap<Id, Rc<dyn Any>>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: HashMap::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn add<S>(&mut self, entity: &Rc<RefCell<Entity>>) {
        self.entities.insert(entity.borrow().id(), entity.clone());
    }

    pub fn get(&self, id: &Id) -> Option<Rc<RefCell<Entity>>> {
        self.entities.get(id).and_then(|e| Some(e.clone()))
    }

    pub fn remove(&mut self, id: Id) {
        self.entities.remove(id.as_ref());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<Box<S>>>)
    where
        S: System,
    {
        self.systems
            .insert(system.borrow().id(), system.clone().as_any());
    }

    pub fn get_system<S>(&self, id: &Id) -> Option<Rc<RefCell<Box<S>>>>
    where
        S: System,
    {
        self.systems
            .get(id)
            .and_then(|s| s.clone().downcast::<RefCell<Box<S>>>().ok())
    }

    pub fn remove_system<S>(&mut self, id: Id) {
        self.systems.remove(id.as_ref());
    }

    pub fn init(&mut self) {
        for s in self.systems.clone().values() {
            s.clone()
                .downcast::<RefCell<Box<dyn System>>>()
                .and_then(|s| {
                    s.borrow_mut().on_init(self);

                    Ok(())
                })
                .ok();
        }
    }

    pub fn update(&mut self) {
        for s in self.systems.clone().values() {
            s.clone()
                .downcast::<RefCell<Box<dyn System>>>()
                .and_then(|s| {
                    s.borrow_mut().on_update(self);

                    Ok(())
                })
                .ok();
        }
    }
}
