use super::{Component, GenericComponent, GenericEntity, GenericSystem, Id, System, ToMut, ToRef};
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

#[derive(Clone)]
pub struct World {
    entities: HashMap<Id, GenericEntity>,
    systems: HashMap<Id, GenericSystem>,
}

impl World {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            entities: HashMap::new(),
            systems: HashMap::new(),
        }))
    }

    pub fn get_entities(&self) -> &HashMap<Id, GenericEntity> {
        &self.entities
    }

    pub fn change_id(&mut self, old: &Id, new: &Id) {
        if let Some((_, e)) = self.remove(old) {
            self.add(&(new.clone(), e));
        };
    }

    pub fn add(&mut self, e @ (id, _): &GenericEntity) {
        self.entities.insert(id.clone(), e.clone());
    }

    pub fn get(&self, id: &Id) -> Option<GenericEntity> {
        Some(self.entities.get(id.as_ref())?.clone())
    }

    pub fn get_all(&self, id: &Id) -> Vec<(GenericEntity, GenericComponent)> {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| Some((p.clone(), e.try_borrow().ok()?.get(id)?.clone())))
            .collect()
    }

    pub fn get_all_ref<C>(&self) -> Vec<(GenericEntity, Ref<C>)>
    where
        C: Component + 'static,
    {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    unsafe { e.try_borrow_unguarded() }.ok()?.get_ref()?,
                ))
            })
            .collect()
    }

    pub fn get_all_mut<C>(&self) -> Vec<(GenericEntity, RefMut<C>)>
    where
        C: Component + 'static,
    {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    unsafe { e.try_borrow_unguarded() }.ok()?.get_ref_mut()?,
                ))
            })
            .collect()
    }

    pub fn get_all_with(&self, ids: &[&Id]) -> Vec<(GenericEntity, Vec<GenericComponent>)> {
        self.entities
            .values()
            .filter_map(|p @ (_, e)| {
                Some((
                    p.clone(),
                    ids.iter()
                        .map(|id| Some(e.try_borrow().ok()?.get(id)?.clone()))
                        .collect::<Option<Vec<_>>>()?,
                ))
            })
            .collect()
    }

    pub fn remove(&mut self, id: &Id) -> Option<GenericEntity> {
        self.entities.remove(id.as_ref())
    }

    pub fn get_systems(&self) -> &HashMap<Id, GenericSystem> {
        &self.systems
    }

    pub fn add_generic_system(&mut self, s @ (id, _): &GenericSystem) {
        self.systems.insert(id.clone(), s.clone());
    }

    pub fn add_system<S>(&mut self, system: &Rc<RefCell<S>>)
    where
        S: System + Component + 'static,
    {
        self.add_generic_system(&(S::get_id(), system.clone()))
    }

    pub fn get_system(&self, id: &Id) -> Option<&GenericSystem> {
        self.systems.get(id)
    }

    pub fn get_system_ref<S>(&self) -> Option<Ref<S>>
    where
        S: Component + System + 'static,
    {
        self.get_system(&S::get_id())
            .and_then(|(_, s)| Ref::filter_map(s.try_borrow().ok()?, |s| s.to_ref()).ok())
    }

    pub fn get_system_ref_mut<S>(&self) -> Option<RefMut<S>>
    where
        S: Component + System + 'static,
    {
        self.get_system(&S::get_id())
            .and_then(|(_, s)| RefMut::filter_map(s.try_borrow_mut().ok()?, |s| s.to_mut()).ok())
    }

    pub fn remove_generic_system(&mut self, id: &Id) -> Option<GenericSystem> {
        self.systems.remove(id.as_ref())
    }

    pub fn remove_system<S>(&mut self) -> Option<GenericSystem>
    where
        S: Component + 'static,
    {
        self.remove_generic_system(&S::get_id())
    }
}
