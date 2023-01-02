use super::component_manager::ComponentManager;
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

    pub fn get(&self, id: usize) -> Option<&BTreeMap<usize, usize>> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut BTreeMap<usize, usize>> {
        self.entities.get_mut(&id)
    }

    pub fn rm(&mut self, id: usize, component_manager: &mut ComponentManager) {
        if let Some(e) = self.entities.remove(&id) {
            for cid in e.values() {
                component_manager.rm_gen(id, *cid, self);
            }
        }
    }
}
