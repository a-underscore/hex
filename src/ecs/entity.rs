use super::{AsAny, Component, Id};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone)]
pub struct Entity {
    components: HashMap<Id, (Id, Rc<RefCell<dyn AsAny>>)>,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn get_components(&self) -> &HashMap<Id, (Id, Rc<RefCell<dyn AsAny>>)> {
        &self.components
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

    pub fn get(&self, id: &Id) -> Option<&(Id, Rc<RefCell<dyn AsAny>>)> {
        self.components.get(id)
    }

    pub fn get_all(&self, ids: &[&Id]) -> Vec<(Id, Rc<RefCell<dyn AsAny>>)> {
        ids.iter()
            .filter_map(|id| self.get(id).and_then(|c| Some(c.clone())))
            .collect()
    }

    pub fn get_ref<C>(&self) -> Option<Ref<C>>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id()).and_then(|(_, c)| {
            Ref::filter_map(c.try_borrow().ok()?, |c| c.as_any_ref().downcast_ref()).ok()
        })
    }

    pub fn get_ref_mut<C>(&self) -> Option<RefMut<C>>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id()).and_then(|(_, c)| {
            RefMut::filter_map(c.try_borrow_mut().ok()?, |c| c.as_any_mut().downcast_mut()).ok()
        })
    }

    pub fn remove_generic(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<dyn AsAny>>)> {
        self.components.remove(id.as_ref())
    }

    pub fn remove<C>(&mut self) -> Option<(Id, Rc<RefCell<dyn AsAny>>)>
    where
        C: Component,
    {
        self.remove_generic(&C::get_id())
    }
}
