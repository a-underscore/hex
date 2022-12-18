use super::{cast_mut, cast_ref, AsAny, Component};
use std::collections::HashMap;

#[derive(Default)]
pub struct Manager<'a> {
    pub entities: HashMap<usize, HashMap<usize, usize>>,
    pub cache: HashMap<usize, Box<dyn AsAny<'a>>>,
    pub freed: Vec<usize>,
}

impl<'a> Manager<'a> {
    pub fn add_c_gen(
        &mut self,
        eid: usize,
        cid: usize,
        component: Box<dyn AsAny<'a>>,
    ) -> Option<usize> {
        let id = self.freed.pop().unwrap_or(self.cache.len());

        self.entities.get_mut(&eid).map(|c| c.insert(cid, id))?;
        self.cache.insert(id, component);

        Some(id)
    }

    pub fn add_c<C>(&mut self, eid: usize, component: C) -> Option<usize>
    where
        C: Component + 'a,
    {
        self.add_c_gen(eid, C::id(), Box::new(component))
    }

    pub fn rm_c_gen(&mut self, eid: usize, cid: usize) {
        if let Some(c) = self.entities.get_mut(&eid).and_then(|c| c.remove(&cid)) {
            self.cache.remove(&c);
            self.freed.push(c);
        }
    }

    pub fn rm_c<C>(&mut self, eid: usize)
    where
        C: Component,
    {
        self.rm_c_gen(eid, C::id());
    }

    pub fn get_c_gen(&self, eid: usize, cid: usize) -> Option<&dyn AsAny<'a>> {
        self.entities
            .get(&eid)
            .and_then(|c| c.get(&cid).map(|cid| *cid))
            .and_then(|cid| self.get_c_gen_cached(cid))
    }

    pub fn get_c<C>(&self, eid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_c_gen(eid, C::id()).map(cast_ref)
    }

    pub fn get_c_gen_mut(&mut self, eid: usize, cid: usize) -> Option<&mut dyn AsAny<'a>> {
        self.entities
            .get_mut(&eid)
            .and_then(|c| c.get_mut(&cid).map(|cid| *cid))
            .and_then(|cid| self.get_c_gen_cached_mut(cid))
    }

    pub fn get_c_mut<C>(&mut self, eid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_c_gen_mut(eid, C::id()).map(|c| cast_mut(c))
    }

    pub fn add_c_gen_cached(&mut self, eid: usize, cmpid: usize, cid: usize) {
        self.entities.get_mut(&eid).map(|c| c.insert(cmpid, cid));
    }

    pub fn add_c_cached<C>(&mut self, eid: usize, cid: usize)
    where
        C: Component,
    {
        self.entities.get_mut(&eid).map(|c| c.insert(C::id(), cid));
    }

    pub fn get_c_gen_cached(&self, cid: usize) -> Option<&dyn AsAny<'a>> {
        self.cache.get(&cid).map(|c| c.as_ref())
    }

    pub fn get_c_cached<C>(&self, cid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_c_gen_cached(cid).map(cast_ref)
    }

    pub fn get_c_gen_cached_mut(&mut self, cid: usize) -> Option<&mut dyn AsAny<'a>> {
        self.cache.get_mut(&cid).map(|c| c.as_mut())
    }

    pub fn get_c_cached_mut<C>(&mut self, cid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_c_gen_cached_mut(cid).map(|c| cast_mut(c))
    }

    pub fn add_e_next(&mut self) -> usize {
        let mut entities = self.entities();

        entities.sort();

        let eid = entities
            .into_iter()
            .enumerate()
            .take_while(|(i, id)| i == id)
            .last()
            .map(|(_, id)| id + 1)
            .unwrap_or(0);

        self.add_e(eid);

        eid
    }

    pub fn add_e(&mut self, eid: usize) {
        self.rm_e(eid);

        self.entities.insert(eid, HashMap::new());
    }

    pub fn get_e(&self, eid: usize) -> Option<&HashMap<usize, usize>> {
        self.entities.get(&eid)
    }

    pub fn get_e_mut(&mut self, eid: usize) -> Option<&mut HashMap<usize, usize>> {
        self.entities.get_mut(&eid)
    }

    pub fn rm_e(&mut self, eid: usize) {
        if let Some(e) = self.entities.remove(&eid) {
            for v in e.values().cloned() {
                self.cache.remove(&v);
                self.freed.push(v);
            }
        }
    }

    pub fn entities(&self) -> Vec<usize> {
        self.entities.keys().cloned().collect()
    }
}
