use super::{ComponentManager, Id};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct EntityManager {
    pub entities: BTreeMap<Id, BTreeMap<Id, Id>>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: Id) {
        self.entities.insert(id, BTreeMap::new());
    }

    pub fn add(&mut self) -> Id {
        let id = self
            .entities
            .keys()
            .cloned()
            .enumerate()
            .find(|(i, id)| *i as Id != *id)
            .map(|(_, id)| id - 1)
            .unwrap_or(self.entities.len() as Id);

        self.add_gen(id);

        id
    }

    pub fn get(&self, eid: Id) -> Option<&BTreeMap<Id, Id>> {
        self.entities.get(&eid)
    }

    pub fn get_mut(&mut self, eid: Id) -> Option<&mut BTreeMap<Id, Id>> {
        self.entities.get_mut(&eid)
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&eid) {
            for cid in e.values() {
                cm.rm_gen(eid, *cid, self);
            }
        }
    }
}
