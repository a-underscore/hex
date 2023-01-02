use super::{
    component_manager::ComponentManager, entity_manager::EntityManager,
    system_manager::SystemManager,
};

#[derive(Default)]
pub struct World<'a, 'b> {
    pub entity_manager: EntityManager,
    pub component_manager: ComponentManager<'a>,
    pub system_manager: SystemManager<'b>,
}
