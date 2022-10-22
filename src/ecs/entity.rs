use super::{Component, GenericComponent, Id, ToMut, ToRef};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone)]
pub struct Entity {
    components: HashMap<Id, GenericComponent>,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn get_components(&self) -> &HashMap<Id, GenericComponent> {
        &self.components
    }

    pub fn add_generic(&mut self, c @ (id, _): &GenericComponent) {
        self.components.insert(id.clone(), c.clone());
    }

    pub fn add<C>(&mut self, component: &Rc<RefCell<C>>)
    where
        C: Component + 'static,
    {
        self.add_generic(&(C::get_id(), component.clone()));
    }

    pub fn get(&self, id: &Id) -> Option<&GenericComponent> {
        self.components.get(id)
    }

    pub fn get_all(&self, ids: &[&Id]) -> Vec<GenericComponent> {
        ids.iter().filter_map(|id| self.get(id).cloned()).collect()
    }

    pub fn get_ref<C>(&self) -> Option<Ref<C>>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id())
            .and_then(|(_, c)| Ref::filter_map(c.try_borrow().ok()?, |c| c.to_ref()).ok())
    }

    pub fn get_ref_mut<C>(&self) -> Option<RefMut<C>>
    where
        C: Component + 'static,
    {
        self.get(&C::get_id())
            .and_then(|(_, c)| RefMut::filter_map(c.try_borrow_mut().ok()?, |c| c.to_mut()).ok())
    }

    pub fn remove_generic(&mut self, id: &Id) -> Option<GenericComponent> {
        self.components.remove(id.as_ref())
    }

    pub fn remove<C>(&mut self) -> Option<GenericComponent>
    where
        C: Component,
    {
        self.remove_generic(&C::get_id())
    }
}

pub type GenericEntity = (Id, Rc<RefCell<Entity>>);
