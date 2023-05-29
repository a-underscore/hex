use super::{id, ComponentManager, Id};
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub entities: HashMap<Id, HashMap<Id, Id>>,
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

            for cid in e.values().cloned() {
                cm.rm_gen(eid, cid, self);
            }
        }
    }
}
