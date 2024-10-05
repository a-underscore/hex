use crate::{world::EntityManager, Id};

#[derive(Clone)]
pub struct Tag(pub String);

impl Tag {
    pub fn new<S>(t: S) -> Self
    where
        S: Into<String>,
    {
        Self(t.into())
    }

    pub fn find(&self, em: &EntityManager) -> Option<Id> {
        em.entities().find_map(|e| {
            em.get_component::<Tag>(e)
                .and_then(|t| (self.0 == t.read().0).then_some(e))
        })
    }
}
