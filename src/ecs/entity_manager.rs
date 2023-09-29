use super::{id, ComponentManager, Id};
use std::{
    any::TypeId,
    collections::{hash_map::Keys, HashMap, HashSet},
    iter::Cloned,
};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub(super) components: HashMap<(Id, TypeId), Id>,
    pub(super) entities: HashMap<Id, HashSet<TypeId>>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: Id) {
        self.entities.insert(id, HashSet::new());
    }

    pub fn add(&mut self) -> Id {
        let id = id::next(&mut self.free, &self.entities);

        self.add_gen(id);

        id
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&eid) {
            self.free.push(eid);

            for cid in e {
                if let Some(id) = self.components.remove(&(eid, cid)) {
                    cm.rm_cache(id);
                }
            }
        }
    }

    pub fn entities(&self) -> Cloned<Keys<Id, HashSet<TypeId>>> {
        self.entities.keys().cloned()
    }
}
