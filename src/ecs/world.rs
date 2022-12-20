use super::{ComponentManager, EntityManager, System};
use glium::{glutin::event::Event, Display};

#[derive(Default)]
pub struct World<'a, 'b> {
    pub entity_manager: EntityManager,
    pub component_manager: ComponentManager<'a>,
    pub systems: Vec<Box<dyn System<'b>>>,
}

impl<'a, 'b> World<'a, 'b> {
    pub fn add_s<S>(&mut self, s: S)
    where
        S: System<'b>,
    {
        self.systems.push(Box::new(s));
    }

    pub fn init(&mut self, display: &Display) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.init(
                &mut self.entity_manager,
                &mut self.component_manager,
                display,
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, display: &Display, event: &Event<()>) -> anyhow::Result<()> {
        for s in &mut self.systems {
            s.update(
                &mut self.entity_manager,
                &mut self.component_manager,
                display,
                event,
            )?;
        }

        Ok(())
    }
}
