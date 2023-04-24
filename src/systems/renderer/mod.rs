use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{uniform, uniforms::Sampler, Display, Surface};

pub struct Renderer {
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl Renderer {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            texture_shader: Shader::new(
                display,
                include_str!("vertex/texture_vertex.glsl"),
                include_str!("fragment/texture_fragment.glsl"),
                None,
            )?,
            color_shader: Shader::new(
                display,
                include_str!("vertex/color_vertex.glsl"),
                include_str!("fragment/color_fragment.glsl"),
                None,
            )?,
        })
    }
}

impl<'a> System<'a> for Renderer {
    fn update(
        &mut self,
        event: &mut Ev,
        _: &mut Scene,
        (em, cm): (&mut EntityManager, &mut ComponentManager),
    ) -> anyhow::Result<()> {
        if let Ev::Draw((_, target)) = event {
            if let Some((c, ct)) = em.entities.keys().cloned().find_map(|e| {
                Some((
                    cm.get::<Camera>(e, em)
                        .and_then(|c| c.active.then_some(c))?,
                    cm.get::<Transform>(e, em)
                        .and_then(|t| t.active.then_some(t))?,
                ))
            }) {
                let models = {
                    let mut models: Vec<_> = em
                        .entities
                        .keys()
                        .cloned()
                        .filter_map(|e| {
                            Some((
                                cm.get::<Model>(e, em).and_then(|s| s.active.then_some(s))?,
                                cm.get::<Transform>(e, em)
                                    .and_then(|t| t.active.then_some(t))?,
                            ))
                        })
                        .collect();

                    models.sort_by(|(_, t1), (_, t2)| {
                        t1.position().z().total_cmp(&t2.position().z())
                    });

                    models
                };

                for (s, t) in models {
                    let (v, i) = &*s.mesh.buffer;

                    match &s.texture {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                color: s.color.0,
                                tex: Sampler(buffer, texture.sampler_behaviour),
                            };

                            target.draw(
                                (v, uv),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &s.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                color: s.color.0,
                            };

                            target.draw(
                                v,
                                i.source(),
                                &self.color_shader.program,
                                &u,
                                &s.draw_parameters,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
