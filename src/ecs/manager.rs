use super::{cast, cast_mut, cast_ref, AsAny, Component};
use std::collections::HashMap;

#[derive(Default)]
pub struct Manager<'a> {
    pub components: HashMap<usize, HashMap<usize, usize>>,
    pub cache: Vec<Box<dyn AsAny<'a>>>,
}

impl<'a> Manager<'a> {
    pub fn add_c_generic(&mut self, eid: usize, cid: usize, component: Box<dyn AsAny<'a>>) {
        let id = self.cache.len();

        self.cache.push(component);

        self.components.get_mut(&eid).map(|c| c.insert(cid, id));
    }

    pub fn add_c<C>(&mut self, eid: usize, component: C)
    where
        C: Component + 'a,
    {
        self.add_c_generic(eid, C::id(), Box::new(component));
    }

    pub fn rm_c_generic(&mut self, eid: usize, cid: usize) -> Option<Box<dyn AsAny<'a>>> {
        self.components
            .get_mut(&eid)
            .and_then(|c| c.remove(&cid))
            .map(|c| self.cache.remove(c))
    }

    pub fn rm_c<C>(&mut self, eid: usize) -> Option<C>
    where
        C: Component,
    {
        self.rm_c_generic(eid, C::id()).map(|c| cast(c))
    }

    pub fn get_c_generic(&self, eid: usize, cid: usize) -> Option<&Box<dyn AsAny<'a>>> {
        self.components
            .get(&eid)
            .and_then(|c| c.get(&cid).map(|c| &self.cache[*c]))
    }

    pub fn get_c<C>(&self, eid: usize) -> Option<&C>
    where
        C: Component,
    {
        self.get_c_generic(eid, C::id()).map(|c| cast_ref(c))
    }

    pub fn get_c_generic_mut(&mut self, eid: usize, cid: usize) -> Option<&mut Box<dyn AsAny<'a>>> {
        self.components
            .get_mut(&eid)
            .and_then(|c| c.get_mut(&cid).map(|c| &mut self.cache[*c]))
    }

    pub fn get_c_mut<C>(&mut self, eid: usize) -> Option<&mut C>
    where
        C: Component,
    {
        self.get_c_generic_mut(eid, C::id()).map(|c| cast_mut(c))
    }

    pub fn add_e(&mut self, eid: usize) {
        self.components.insert(eid, HashMap::new());
    }

    pub fn get_e<'b>(&'b self, eid: usize) -> Option<&'b HashMap<usize, usize>>
    where
        'a: 'b,
    {
        self.components.get(&eid)
    }

    pub fn get_e_mut<'b>(&'b mut self, eid: usize) -> Option<&'b mut HashMap<usize, usize>>
    where
        'a: 'b,
    {
        self.components.get_mut(&eid)
    }

    pub fn rm_e(&mut self, eid: usize) {
        self.components.remove(&eid).map(|e| {
            for v in e.values() {
                self.rm_c_generic(eid, *v);
            }
        });
    }

    pub fn entities(&self) -> Vec<usize> {
        self.components.keys().cloned().collect()
    }
}
