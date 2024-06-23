use super::{ComponentManager, Id};
use std::{
    any::TypeId,
    collections::{
        hash_map::{Entry, Iter},
        HashMap, HashSet,
    },
    iter::FilterMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct EntityManager {
    free: Vec<Id>,
    entities: HashMap<Id, (bool, HashSet<TypeId>)>,
}

impl EntityManager {
    pub fn new() -> Arc<RwLock<Self>> {
        Default::default()
    }

    pub fn add_gen(&mut self, id: Id, active: bool) {
        self.entities.insert(id, (active, HashSet::new()));
    }

    pub fn add(&mut self, active: bool) -> Id {
        let id = self.free.pop().unwrap_or(self.entities.len() as Id);

        self.add_gen(id, active);

        id
    }

    pub fn rm(&mut self, eid: Id, cm: &mut ComponentManager) {
        if let Entry::Occupied(e) = self.entities.entry(eid) {
            let a = {
                let (a, _) = e.get();

                *a
            };

            if a {
                let (_, e) = e.remove();

                self.free.push(eid);

                for cid in e {
                    cm.components.remove(&(eid, cid));
                }
            }
        }
    }

    pub fn get(&self, eid: Id) -> Option<&HashSet<TypeId>> {
        self.entities.get(&eid).and_then(|(a, e)| a.then_some(e))
    }

    pub fn get_mut(&mut self, eid: Id) -> Option<&mut HashSet<TypeId>> {
        self.entities
            .get_mut(&eid)
            .and_then(|(a, e)| a.then_some(e))
    }

    pub fn set_active(&mut self, eid: Id, active: bool) {
        if let Some((a, _)) = self.entities.get_mut(&eid) {
            *a = active;
        }
    }

    pub fn activate(&mut self, eid: Id) {
        self.set_active(eid, true);
    }

    pub fn deactivate(&mut self, eid: Id) {
        self.set_active(eid, false);
    }

    pub fn entities(
        &self,
    ) -> FilterMap<
        Iter<'_, Id, (bool, HashSet<TypeId>)>,
        for<'a> fn((&'a Id, &'a (bool, HashSet<TypeId>))) -> Option<Id>,
    > {
        self.entities
            .iter()
            .filter_map(|(e, (a, _))| a.then_some(*e))
    }
}
