use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::Draw,
    renderer_manager::Renderer,
    ComponentManager, Context, EntityManager,
};
use std::sync::{Arc, RwLock};

pub struct SpriteRenderer;

impl Renderer for SpriteRenderer {
    fn draw(
        &mut self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        entity_manager: Arc<RwLock<EntityManager>>,
        component_manager: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        let context = context.read().unwrap();
        let em = entity_manager.read().unwrap();
        let cm = component_manager.read().unwrap();

        if let Some((ce, c, ct)) = em.entities().keys().cloned().find_map(|e| {
            Some((
                e,
                cm.get::<Camera>(e)
                    .and_then(|c| c.read().unwrap().active.then_some(c))?,
                cm.get::<Trans>(e)
                    .and_then(|t| t.read().unwrap().active.then_some(t))?,
            ))
        }) {
            let sprites = {
                let mut sprites: Vec<_> = em
                    .entities()
                    .keys()
                    .cloned()
                    .filter_map(|e| {
                        Some((
                            e,
                            cm.get::<Sprite>(e)
                                .and_then(|s| s.read().unwrap().active.then_some(s))?,
                            cm.get::<Trans>(e)
                                .and_then(|t| t.read().unwrap().active.then_some(t))?,
                        ))
                    })
                    .collect();

                sprites.sort_by(|(_, s1, _), (_, s2, _)| {
                    s1.read().unwrap().layer.cmp(&s2.read().unwrap().layer)
                });

                sprites
            };

            for (se, s, t) in sprites {
                let d = {
                    let s = s.write().unwrap();

                    s.drawable.clone()
                };

                d.draw(
                    (se, t.clone(), s.clone()),
                    (ce, c.clone(), ct.clone()),
                    &context,
                    draw,
                    entity_manager.clone(),
                    component_manager.clone(),
                )?;
            }
        }

        Ok(())
    }
}
