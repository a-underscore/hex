use std::collections::HashMap;
use super::ComponentManager;

#[derive(Default)]
pub struct EntityManager {
    pub entities: HashMap<usize, usize>,
}

impl EntityManager {
    pub fn add<'b>(&'b self, eid: usize)
    {
        let cid = self.components.len();

        self.entities.insert(eid, cid);

        self.components.push(ComponentManager::default());
    }

    pub fn get<'b>(&'b self, eid: usize) -> Option<usize>
    {
        self.entities.get(&eid);
    }

    pub fn entities(&self) -> Vec<usize> {
        self.entities.keys().cloned().collect()
    }
}
