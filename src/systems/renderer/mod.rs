use crate::{
    assets::Shader,
    components::{Camera, Model, Transform},
    ecs::{system_manager::System, ComponentManager, EntityManager, Ev, Scene},
};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    uniform,
    uniforms::Sampler,
    Depth, Display, DrawParameters, Surface,
};

pub struct Renderer<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub texture_shader: Shader,
    pub color_shader: Shader,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        Ok(Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            texture_shader: Shader::new(
                display,
                include_str!("texture/vertex.glsl"),
                include_str!("texture/fragment.glsl"),
                None,
            )?,
            color_shader: Shader::new(
                display,
                include_str!("color/vertex.glsl"),
                include_str!("color/fragment.glsl"),
                None,
            )?,
        })
    }
}

impl<'a> System<'a> for Renderer<'a> {
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
                        (ct.position() - t1.position())
                            .magnitude()
                            .total_cmp(&(ct.position() - t2.position()).magnitude())
                    });

                    models
                };

                for (m, t) in models {
                    let (mesh, ma, texture) = &*m.data;
                    let (v, i) = &*mesh.buffer;

                    match texture {
                        Some(texture) => {
                            let (uv, buffer) = &*texture.buffer;
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                buffer: Sampler(buffer, texture.sampler_behaviour),
                                color: ma.color.0,
                            };

                            target.draw(
                                (v, uv),
                                i.source(),
                                &self.texture_shader.program,
                                &u,
                                &self.draw_parameters,
                            )?;
                        }
                        None => {
                            let u = uniform! {
                                transform: t.matrix().0,
                                camera_transform: ct.matrix().0,
                                camera_view: c.view().0,
                                color: ma.color.0,
                            };

                            target.draw(
                                v,
                                i.source(),
                                &self.color_shader.program,
                                &u,
                                &self.draw_parameters,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
