use super::{ComponentManager, Id};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    pub(super) entities: HashMap<Id, HashSet<TypeId>>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: Id) {
        self.entities.insert(id, HashSet::new());
    }

    pub fn add(&mut self) -> Id {
        let id = self.free.pop().unwrap_or(self.entities.len() as Id);

        self.add_gen(id);

        id
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&eid) {
            self.free.push(eid);

            for cid in e {
                cm.components.remove(&(eid, cid));
            }
        }
    }

    pub fn get(&self, eid: Id) -> Option<&HashSet<TypeId>> {
        self.entities.get(&eid)
    }

    pub fn entities(&self) -> &HashMap<Id, HashSet<TypeId>> {
        &self.entities
    }
}
