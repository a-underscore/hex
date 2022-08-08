use crate::{Component, Id};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

pub struct Entity {
    components: HashMap<Id, Rc<dyn Any>>,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn add<C>(&mut self, component: &Rc<RefCell<Box<C>>>)
    where
        C: Component,
    {
        self.components
            .insert(component.borrow().id(), component.clone() as Rc<dyn Any>);
    }

    pub fn get<C>(&self, id: &Id) -> Option<Rc<RefCell<Box<C>>>>
    where
        C: Component,
    {
        self.components
            .get(id)
            .and_then(|c| c.clone().downcast::<RefCell<Box<C>>>().ok())
    }

    pub fn remove(&mut self, id: &Id) {
        self.components.remove(id.as_ref());
    }
}
