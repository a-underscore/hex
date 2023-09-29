use super::{id, ComponentManager, Id};
use std::{
    any::TypeId,
    collections::{hash_map::Keys, HashMap},
    iter::Cloned,
};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub(super) entities: HashMap<Id, HashMap<TypeId, Id>>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: Id) {
        self.entities.insert(id, HashMap::new());
    }

    pub fn add(&mut self) -> Id {
        let id = id::next(&mut self.free, &self.entities);

        self.add_gen(id);

        id
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&eid) {
            self.free.push(eid);

            for id in e.into_values() {
                cm.rm_cache(id);
            }
        }
    }

    pub fn entities(&self) -> Cloned<Keys<Id, HashMap<TypeId, Id>>> {
        self.entities.keys().cloned()
    }
}
