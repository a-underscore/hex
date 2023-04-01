use super::{ComponentManager, EntityManager};

#[derive(Default)]
pub struct World<'a> {
    pub em: EntityManager,
    pub cm: ComponentManager<'a>,
}
