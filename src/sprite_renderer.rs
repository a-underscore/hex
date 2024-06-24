use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::{Draw, Renderer},
    ComponentManager, Context, EntityManager,
};
use std::sync::{Arc, RwLock};

pub struct SpriteRenderer;

impl Renderer for SpriteRenderer {
    fn draw(
        &mut self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        if let Some(((ce, ct, c), sprites)) = {
            let em = em.read().unwrap();
            let cm = cm.read().unwrap();

            if let Some(c) = em
                .entities()
                .find_map(|e| Some((e, cm.get::<Trans>(e)?.clone(), cm.get::<Camera>(e)?.clone())))
            {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((e, cm.get::<Trans>(e)?.clone(), cm.get::<Sprite>(e)?.clone()))
                        })
                        .collect();

                    sprites.sort_by(|(_, _, s1), (_, _, s2)| {
                        s1.read().unwrap().layer.cmp(&s2.read().unwrap().layer)
                    });

                    sprites
                };
                Some((c, sprites))
            } else {
                None
            }
        } {
            for (se, t, s) in sprites {
                let d = s.read().unwrap().drawable.clone();

                d.write().unwrap().draw(
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
