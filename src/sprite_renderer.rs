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
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        let context = context.read().unwrap();
        let em = em.read().unwrap();
        let cm = cm.read().unwrap();

        if let Some((ce, ct, c)) = em
            .entities()
            .keys()
            .cloned()
            .find_map(|e| Some((e, cm.get::<Trans>(e)?, cm.get::<Camera>(e)?)))
        {
            let sprites = {
                let mut sprites: Vec<_> = em
                    .entities()
                    .keys()
                    .cloned()
                    .filter_map(|e| Some((e, cm.get::<Trans>(e)?, cm.get::<Sprite>(e)?)))
                    .collect();

                sprites.sort_by(|(_, _, s1), (_, _, s2)| {
                    s1.read().unwrap().layer.cmp(&s2.read().unwrap().layer)
                });

                sprites
            };

            for (se, t, s) in sprites {
                let d = s.read().unwrap().drawable.clone();

                d.draw(
                    (se, t.clone(), s.clone()),
                    (ce, ct.clone(), c.clone()),
                    &context,
                    draw,
                    &em,
                    &cm,
                )?;
            }
        }

        Ok(())
    }
}
