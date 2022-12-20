use super::ComponentManager;
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityManager {
    pub entities: HashMap<usize, HashMap<usize, usize>>,
    freed: Vec<usize>,
}

impl EntityManager {
    pub fn add_gen(&mut self, id: usize) {
        self.entities.insert(id, HashMap::new());
    }

    pub fn add(&mut self) -> usize {
        let id = self.freed.pop().unwrap_or(self.entities.len());

        self.add_gen(id);

        id
    }

    pub fn get(&self, id: usize) -> Option<&HashMap<usize, usize>> {
        self.entities.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut HashMap<usize, usize>> {
        self.entities.get_mut(&id)
    }

    pub fn rm(&mut self, component_manager: &mut ComponentManager, id: usize) {
        if let Some(e) = self.entities.remove(&id) {
            self.freed.push(id);

            for cid in e.values() {
                component_manager.rm_gen(self, id, *cid);
            }
        }
    }
}
