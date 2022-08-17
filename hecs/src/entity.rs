use crate::{Component, Id};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

pub struct Entity {
    components: HashMap<Id, Rc<RefCell<dyn Component>>>,
}

impl Entity {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            components: HashMap::new(),
        }))
    }

    pub fn add_generic(&mut self, component: Rc<RefCell<dyn Component>>) {
        self.components
            .insert(component.borrow().get_id(), component.clone());
    }

    pub fn add<C>(&mut self, component: &Rc<RefCell<C>>)
    where
        C: Component,
    {
        self.add_generic(component.clone());
    }

    pub fn get<'a>(&'a self, id: &Id) -> Option<(Id, &'a Rc<RefCell<dyn Component>>)> {
        self.components.get(id).and_then(|c| Some((id.clone(), c)))
    }

    pub fn get_all(&self, ids: &[&Id]) -> Option<Vec<(Id, Rc<RefCell<dyn Component>>)>> {
        ids.iter()
            .cloned()
            .map(|id| {
                self.components
                    .get(id)
                    .and_then(|c| Some((id.clone(), c.clone())))
            })
            .collect()
    }

    pub fn get_ref<C>(&self, id: &Id) -> Option<(Id, Ref<C>)>
    where
        C: Component,
    {
        self.get(id).and_then(|(id, c)| {
            Some((
                id.clone(),
                Ref::filter_map(c.borrow(), |c| c.as_any_ref().downcast_ref::<C>()).ok()?,
            ))
        })
    }

    pub fn get_mut<C>(&self, id: &Id) -> Option<(Id, RefMut<C>)>
    where
        C: Component,
    {
        self.get(id).and_then(|(id, c)| {
            Some((
                id.clone(),
                RefMut::filter_map(c.borrow_mut(), |c| c.as_any_mut().downcast_mut::<C>()).ok()?,
            ))
        })
    }

    pub fn remove(&mut self, id: &Id) -> Option<(Id, Rc<RefCell<dyn Component>>)> {
        self.components
            .remove(id.as_ref())
            .and_then(|c| Some((id.clone(), c)))
    }

    pub fn get_components<'a>(&'a self) -> &'a HashMap<Id, Rc<RefCell<dyn Component>>> {
        &self.components
    }
}
