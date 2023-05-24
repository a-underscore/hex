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
        let id = id::next(&self.entities, &mut self.free);

        self.add_gen(id);

        id
    }

    pub fn get(&self, eid: Id) -> Option<&HashMap<Id, Id>> {
        self.entities.get(&eid)
    }

    pub fn get_mut(&mut self, eid: Id) -> Option<&mut HashMap<Id, Id>> {
        self.entities.get_mut(&eid)
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
