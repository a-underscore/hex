pub mod as_any;
pub mod component;

pub use as_any::AsAny;
pub use component::Component;

use super::{EntityManager, Id};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{any::TypeId, collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct ComponentManager {
    pub(super) components: HashMap<(Id, TypeId), Box<dyn AsAny>>,
}

impl ComponentManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Default::default()
    }

    pub fn add_gen(
        &mut self,
        eid: Id,
        cid: TypeId,
        component: Box<dyn AsAny>,
        em: &mut EntityManager,
    ) {
        if let Some((_, components)) = em.entities.get_mut(&eid) {
            components.insert(cid);

            self.components.insert((eid, cid), component);
        }
    }

    pub fn add<C: Component>(&mut self, eid: Id, component: C, em: &mut EntityManager) {
        self.add_gen(
            eid,
            TypeId::of::<C>(),
            Box::new(Arc::new(RwLock::new(component))),
            em,
        );
    }

    pub fn rm_gen(&mut self, eid: Id, cid: TypeId, em: &mut EntityManager) {
        if let Some((_, components)) = em.entities.get_mut(&eid) {
            components.remove(&cid);

            self.components.remove(&(eid, cid));
        }
    }

    pub fn rm<C: Component>(&mut self, eid: Id, em: &mut EntityManager) {
        self.rm_gen(eid, TypeId::of::<C>(), em);
    }

    pub fn get_gen(&self, eid: Id, cid: TypeId) -> Option<&dyn AsAny> {
        self.components.get(&(eid, cid)).map(|c| c.as_ref())
    }

    pub fn get<C: Component>(&self, eid: Id) -> Option<&Arc<RwLock<C>>> {
        self.get_gen(eid, TypeId::of::<C>()).and_then(Self::cast)
    }

    pub fn get_ref<C: Component>(&self, eid: Id) -> Option<RwLockReadGuard<C>> {
        self.get::<C>(eid).map(|c| c.read())
    }

    pub fn get_mut<C: Component>(&self, eid: Id) -> Option<RwLockWriteGuard<C>> {
        self.get::<C>(eid).map(|c| c.write())
    }

    fn cast<C: Component>(a: &dyn AsAny) -> Option<&Arc<RwLock<C>>> {
        a.as_any().downcast_ref()
    }
}
