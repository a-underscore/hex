use crate::{AsAny, Component, Id};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub struct Entity {
    pub components: HashMap<Id, (Id, Rc<RefCell<dyn AsAny>>)>,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn add_generic(&mut self, c @ (id, _): &(Id, Rc<RefCell<dyn AsAny>>)) {
        self.components.insert(id.clone(), c.clone());
    }

    pub fn add<C>(&mut self, component: &Rc<RefCell<C>>)
    where
        C: Component + 'static,
    {
        self.add_generic(&(C::get_id(), component.clone()));
    }

    pub fn get<'a>(&'a self, id: &Id) -> Option<&'a (Id, Rc<RefCell<dyn AsAny>>)> {
        self.components.get(id)
    }

    pub fn get_all(&self, ids: &[&Id]) -> Option<Vec<(Id, Rc<RefCell<dyn AsAny>>)>> {
        ids.iter()
            .map(|id| self.get(id).and_then(|c| Some(c.clone())))
            .collect()
    }

    pub fn get_ref<C>(&self) -> Option<(Id, Ref<C>)>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id()).and_then(|(id, c)| {
            Some((
                id.clone(),
                Ref::filter_map(c.borrow(), |c| c.as_any_ref().downcast_ref::<C>()).ok()?,
            ))
        })
    }

    pub fn get_mut<C>(&self) -> Option<(Id, RefMut<C>)>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id()).and_then(|(id, c)| {
            Some((
                id.clone(),
                RefMut::filter_map(c.borrow_mut(), |c| c.as_any_mut().downcast_mut::<C>()).ok()?,
            ))
        })
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<dyn AsAny>>)> {
        self.components.remove(id.as_ref())
    }

    pub fn get_components<'a>(&'a self) -> &'a HashMap<Id, (Id, Rc<RefCell<dyn AsAny>>)> {
        &self.components
    }
}
