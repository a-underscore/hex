use crate::{
    assets::Shader,
    components::{Camera, Sprite, Transform},
    ecs::{system_manager::System, ComponentManager, Context, EntityManager, Ev},
};
use glium::{
    draw_parameters::{Blend, DepthTest},
    index::NoIndices,
    uniform,
    uniforms::Sampler,
    Depth, Display, DrawParameters, Surface,
};

pub struct Renderer {
    pub draw_parameters: DrawParameters<'static>,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
            shader: Shader::new(
                display,
                include_str!("vertex.glsl"),
                include_str!("fragment.glsl"),
                None,
            )?,
        })
    }
}

impl System for Renderer {
    fn update(
        &mut self,
        ev: &mut Ev,
        _: &mut Context,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = ev {
            if let Some((c, ct)) = em.entities().find_map(|e| {
                Some((
                    cm.get::<Camera>(e, em)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e, em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let sprites = {
                    let mut sprites: Vec<_> = em
                        .entities()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Sprite>(e, em)
                                    .and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e, em)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    sprites.sort_by(|(s1, _), (s2, _)| s1.z.total_cmp(&s2.z));

                    sprites
                };

                for (s, t) in sprites {
                    let uniform = uniform! {
                        z: s.z,
                        transform: t.matrix().0,
                        camera_transform: ct.matrix().0,
                        camera_proj: c.proj().0,
                        color: s.color,
                        tex: Sampler(&*s.texture.buffer, s.texture.sampler_behaviour),
                    };

                    target.draw(
                        &*s.shape.vertices,
                        NoIndices(s.shape.format),
                        &self.shader.program,
                        &uniform,
                        &self.draw_parameters,
                    )?;
                }
            }
        }

        Ok(())
    }
}
