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
    pub(super) components: HashMap<(Id, TypeId), Box<dyn AsAny>>,
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

            self.components.insert((eid, cid), component);
        }
    }

    pub fn add<C>(&mut self, eid: Id, component: C, em: &mut EntityManager)
    where
        C: Component,
    {
        self.add_gen(
            eid,
            TypeId::of::<C>(),
            Box::new(Rc::new(RefCell::new(component))),
            em,
        );
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

    pub fn get_gen(&self, eid: Id, cid: TypeId) -> Option<&dyn AsAny> {
        self.components.get(&(eid, cid)).map(|a| a.as_ref())
    }

    pub fn get<C>(&self, eid: Id, cid: TypeId) -> Option<&Rc<RefCell<C>>>
    where
        C: Component,
    {
        self.get_gen(eid, cid).and_then(Self::cast)
    }

    pub fn get_ref<C>(&self, eid: Id) -> Option<Ref<'_, C>>
    where
        C: Component,
    {
        self.get(eid, TypeId::of::<C>())
            .and_then(|c| c.try_borrow().ok())
    }

    pub fn get_mut<C>(&self, eid: Id) -> Option<RefMut<'_, C>>
    where
        C: Component,
    {
        self.get(eid, TypeId::of::<C>())
            .and_then(|c| c.try_borrow_mut().ok())
    }

    pub fn cast<C>(a: &dyn AsAny) -> Option<&Rc<RefCell<C>>>
    where
        C: Component,
    {
        a.as_any().downcast_ref()
    }
}
