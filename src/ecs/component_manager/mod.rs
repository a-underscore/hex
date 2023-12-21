pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use super::{EntityManager, Id};
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

#[derive(Default)]
pub struct ComponentManager {
    pub(super) components: HashMap<(Id, TypeId), Rc<RefCell<Box<dyn AsAny>>>>,
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

            self.components
                .insert((eid, cid), Rc::new(RefCell::new(component)));
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

    pub fn get(&self, eid: Id, cid: TypeId) -> Option<&Rc<RefCell<Box<dyn AsAny>>>> {
        self.components.get(&(eid, cid))
    }

    pub fn get_ref<C>(&self, eid: Id) -> Option<Ref<'_, C>>
    where
        C: Component,
    {
        self.get(eid, TypeId::of::<C>())
            .and_then(|c| Self::cast(c.try_borrow().ok()?))
    }

    pub fn get_mut<C>(&mut self, eid: Id) -> Option<RefMut<'_, C>>
    where
        C: Component,
    {
        self.get(eid, TypeId::of::<C>())
            .and_then(|c| Self::cast_mut(c.try_borrow_mut().ok()?))
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
