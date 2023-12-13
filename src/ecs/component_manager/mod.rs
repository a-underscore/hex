pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use super::{EntityManager, Id};
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

#[derive(Default)]
pub struct ComponentManager {
    pub(super) components: HashMap<(Id, TypeId), RefCell<Box<dyn AsAny>>>,
}

impl ComponentManager {
    pub fn add_gen(
        &mut self,
        eid: Id,
        cid: TypeId,
        component: Box<dyn AsAny>,
        em: &mut EntityManager,
    ) {
        if let Some(entity) = em.entities.get_mut(&eid) {
            entity.insert(cid);

            self.components.insert((eid, cid), RefCell::new(component));
        }
    }

    pub fn add<C>(&mut self, eid: Id, component: C, em: &mut EntityManager)
    where
        C: Component,
    {
        self.add_gen(eid, TypeId::of::<C>(), Box::new(component), em);
    }

    pub fn rm_gen(&mut self, eid: Id, cid: TypeId, em: &mut EntityManager) {
        if let Some(components) = em.entities.get_mut(&eid) {
            components.remove(&cid);

            self.components.remove(&(eid, cid));
        }
    }

    pub fn rm<C>(&mut self, eid: Id, em: &mut EntityManager)
    where
        C: Component,
    {
        self.rm_gen(eid, TypeId::of::<C>(), em);
    }

    pub fn get_gen(&self, eid: Id, cid: TypeId) -> Option<Ref<'_, Box<dyn AsAny>>> {
        self.components
            .get(&(eid, cid))
            .and_then(|c| c.try_borrow().ok())
    }

    pub fn get<C>(&self, eid: Id) -> Option<Ref<'_, C>>
    where
        C: Component,
    {
        self.get_gen(eid, TypeId::of::<C>()).and_then(Self::cast)
    }

    pub fn get_gen_mut(&self, eid: Id, cid: TypeId) -> Option<RefMut<'_, Box<dyn AsAny>>> {
        self.components
            .get(&(eid, cid))
            .and_then(|c| c.try_borrow_mut().ok())
    }

    pub fn get_mut<C>(&self, eid: Id) -> Option<RefMut<'_, C>>
    where
        C: Component,
    {
        self.get_gen_mut(eid, TypeId::of::<C>())
            .and_then(Self::cast_mut)
    }

    pub fn cast<C>(a: Ref<'_, Box<dyn AsAny>>) -> Option<Ref<'_, C>>
    where
        C: Component,
    {
        Ref::filter_map(a, |a| a.as_any().downcast_ref()).ok()
    }

    pub fn cast_mut<C>(a: RefMut<'_, Box<dyn AsAny>>) -> Option<RefMut<'_, C>>
    where
        C: Component,
    {
        RefMut::filter_map(a, |a| a.as_any_mut().downcast_mut()).ok()
    }
}
