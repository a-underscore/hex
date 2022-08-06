use crate::{Entity, Id, System};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct World {
    pub root: Rc<Entity>,
    pub systems: HashMap<Id, Rc<dyn System>>,
}

impl World {
    pub fn new(root: Rc<Entity>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            root,
            systems: HashMap::new(),
        }))
    }

    pub fn add_system<S>(&mut self, system: &Rc<S>)
    where
        S: System,
    {
        self.systems
            .insert(system.id(), system.clone() as Rc<dyn System>);
    }

    pub fn get_system<S>(&self, id: Id) -> Option<Rc<S>>
    where
        S: System,
    {
        self.systems
            .get(id.as_ref())
            .and_then(|s| s.clone().as_any().downcast::<S>().ok())
    }

    pub fn remove_system<S>(&mut self, id: Id)
    where
        S: System,
    {
        self.systems.remove(id.as_ref());
    }
}
