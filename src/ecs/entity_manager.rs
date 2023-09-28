use super::{id, ComponentManager, Id};
use std::{any::TypeId, collections::HashMap};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub entities: HashMap<Id, HashMap<TypeId, Id>>,
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
        if let Some(e) = self.entities.get(&eid) {
            self.free.push(eid);

            for cid in e.values().cloned() {
                cm.cache.remove(&cid);
            }

            self.entities.remove(&eid);
        }
    }
}
