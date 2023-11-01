use crate::{
    components::{Camera, Sprite, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};

#[derive(Default)]
pub struct Renderer;

impl System for Renderer {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, _)) = ev {
            if let Some((_c, _ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get::<Camera>(e).and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Sprite>(e).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e).and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                for (_s, _t) in sprites {}
            }
        }

        Ok(())
    }
}
