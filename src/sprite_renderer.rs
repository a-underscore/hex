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
                .find_map(|e| Some((e, cm.get::<Trans>(e)?.clone(), cm.get::<Camera>(e)?.clone())))
                .map(|c| {
                    let sprites = {
                        let mut sprites: Vec<_> = em
                            .entities()
                            .filter_map(|e| {
                                Some((e, cm.get::<Trans>(e)?.clone(), cm.get::<Sprite>(e)?.clone()))
                            })
                            .collect();

                        sprites.sort_by_key(|(_, _, s)| s.read().layer);

                        sprites
                    };

                    (c, sprites)
                })
        };

        if let Some(((ce, ct, c), sprites)) = res {
            for (se, t, s) in sprites {
                let d = s.read().drawable.clone();

                d.write().draw(
                    (se, t.clone(), s.clone()),
                    (ce, ct.clone(), c.clone()),
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
