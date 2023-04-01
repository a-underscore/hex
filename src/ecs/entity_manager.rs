use super::ComponentManager;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct EntityManager {
    pub entities: BTreeMap<usize, BTreeMap<usize, usize>>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: usize) {
        self.entities.insert(id, BTreeMap::new());
    }

    pub fn add(&mut self) -> usize {
        let id = self
            .entities
            .keys()
            .cloned()
            .enumerate()
            .find(|(i, id)| *i != *id)
            .map(|(_, id)| id - 1)
            .unwrap_or(self.entities.len());

        self.add_gen(id);

        id
    }

    pub fn get(&self, eid: usize) -> Option<&BTreeMap<usize, usize>> {
        self.entities.get(&eid)
    }

    pub fn get_mut(&mut self, eid: usize) -> Option<&mut BTreeMap<usize, usize>> {
        self.entities.get_mut(&eid)
    }

    pub fn rm(&mut self, eid: usize, cm: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&eid) {
            for cid in e.values() {
                cm.rm_gen(eid, *cid, self);
            }
        }
    }
}
