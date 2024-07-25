use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::{Draw, Renderer},
    ComponentManager, Context, EntityManager,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct SpriteRenderer;

impl Renderer for SpriteRenderer {
    fn draw(
        &mut self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        let res = {
            let em = em.read();
            let cm = cm.read();

            em.entities()
                .find_map(|e| Some((e, cm.get::<Camera>(e)?.clone(), cm.get::<Trans>(e)?.clone())))
                .map(|c| {
                    let sprites = {
                        let mut sprites: Vec<_> = em
                            .entities()
                            .filter_map(|e| {
                                Some((e, cm.get::<Sprite>(e)?.clone(), cm.get::<Trans>(e)?.clone()))
                            })
                            .collect();

                        sprites.sort_by_key(|(_, s, _)| s.read().layer);

                        sprites
                    };

                    (c, sprites)
                })
        };

        if let Some(((ce, c, ct), sprites)) = res {
            for (se, s, t) in sprites {
                let d = s.read().drawable.clone();

                d.write().draw(
                    (se, s.clone(), t.clone()),
                    (ce, c.clone(), ct.clone()),
                    draw,
                    context.clone(),
                    em.clone(),
                    cm.clone(),
                )?;
            }
        }

        Ok(())
    }
}
